import argparse
import json
import re
from enum import Enum, unique
from pathlib import Path

import pandas as pd
from pydantic import BaseModel

# Matches residual accession suffixes like _CF000153485.2.1 on species names.
ACCESSION_SUFFIX_PAT = re.compile(r"_[A-Z0-9]+\.[0-9]+\.[0-9]+$")

# Matches the single-letter taxonomic level prefix (e.g. "d:", "s:").
LEVEL_PREFIX_PAT = re.compile(r"^[dkpcofgs]:")


@unique
class Levels(Enum):
    """Taxonomic levels ordered from most specific to least specific.

    This ordering is intentional: SINTAX classification picks the most specific
    level that exceeds the bootstrap confidence threshold.
    """

    SPECIES = "species"
    GENUS = "genus"
    FAMILY = "family"
    ORDER = "order"
    CLASS = "class"
    PHYLUM = "phylum"
    KINGDOM = "kingdom"

    @classmethod
    def as_list(cls) -> list[str]:
        return [c.value for c in cls]


class SintaxAsvResult(BaseModel):
    level: str
    tax_name: str
    score: float


class SintaxResult(BaseModel):
    result: dict[str, SintaxAsvResult]


def add_taxonomy(df: pd.DataFrame) -> pd.DataFrame:
    tax = (
        df["reference"]
        .str.split(";", expand=True)[2]
        .str.removeprefix("taxonomy=")
        .str.split("|", expand=True)
        .rename(
            columns={
                0: Levels.KINGDOM.value,
                1: Levels.PHYLUM.value,
                2: Levels.CLASS.value,
                3: Levels.ORDER.value,
                4: Levels.FAMILY.value,
                5: Levels.GENUS.value,
                6: Levels.SPECIES.value,
            }
        )
    )

    # Replace literal "nan" taxon names (e.g. "g:nan") with actual NaN
    # so they are excluded from vote counting.
    for level in Levels.as_list():
        tax[level] = tax[level].replace(
            to_replace=r"^[dkpcofgs]:nan$", value=pd.NA, regex=True
        )

    return pd.concat([df, tax], axis=1)


def parse_tsv(tsv: Path, threshold: float, outdir: Path) -> None:
    df = pd.read_csv(
        tsv, sep="\t", names=["asv", "reference", "num_hits", "iteration"]
    )

    df = add_taxonomy(df)

    # Remove residual accession suffixes from species names.
    df["species"] = df["species"].str.replace(ACCESSION_SUFFIX_PAT, "", regex=True)

    result_full: list[list[str]] = []
    asv_short: dict[str, SintaxAsvResult] = {}

    for asv, asv_subset in df.groupby(by="asv"):
        asv_full: list[str] = []
        n_iterations = len(asv_subset)

        for level in Levels.as_list():
            level_values = asv_subset[level].dropna()
            if level_values.empty:
                asv_full.append(f"unclassified(0.0000)")
                continue

            counts = level_values.value_counts()
            best_taxon = counts.index[0]
            best_count = counts.iloc[0]

            best_taxon = LEVEL_PREFIX_PAT.sub("", best_taxon)
            best_score = float(best_count) / n_iterations
            asv_full.append(f"{best_taxon}({best_score:.4f})")

            if asv not in asv_short and best_score >= threshold:
                asv_short[asv] = SintaxAsvResult(
                    level=level, tax_name=best_taxon, score=best_score
                )

        result_full.append([str(asv), "|".join(asv_full)])

    # Write full results.
    result_df = pd.DataFrame(result_full, columns=["asv", "scores"])
    result_df.to_csv(outdir / "result_full.tsv", sep="\t", index=False)

    # Write most specific taxonomic level passing threshold, per ASV.
    with (outdir / "result.json").open("w") as f:
        json.dump(SintaxResult(result=asv_short).model_dump(), f, indent=4)


def main() -> None:
    parser = argparse.ArgumentParser("Parse sintax tsv output from sintax_rs")
    parser.add_argument("--tsv", help="Path to sintax tsv file.", required=True)
    parser.add_argument(
        "--threshold",
        help="Cutoff for determining taxonomic level classification",
        type=float,
        default=0.80,
    )
    parser.add_argument("-o", "--outdir", help="Output directory", required=True)
    args = parser.parse_args()

    tsv = Path(args.tsv)
    if not tsv.is_file():
        raise FileNotFoundError(tsv)

    threshold: float = args.threshold
    if not 0.0 <= threshold <= 1.0:
        msg = f"Threshold {threshold} must be between 0.0 and 1.0."
        raise ValueError(msg)

    outdir = Path(args.outdir)
    outdir.mkdir(exist_ok=True, parents=True)

    parse_tsv(tsv, threshold, outdir)


if __name__ == "__main__":
    main()
