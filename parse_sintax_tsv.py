import argparse
import json
import re
from enum import Enum, unique
from pathlib import Path

import pandas as pd
from pydantic import BaseModel

pat = re.compile(r'_[A-Z0-9]+\.[0-9]+\.[0-9]+$')


@unique
class Levels(Enum):
    SPECIES = 'species'
    GENUS = 'genus'
    FAMILY = 'family'
    ORDER = 'order'
    CLASS = 'class'
    PHYLUM = 'phylum'
    KINGDOM = 'kingdom'

    @classmethod
    def as_list(cls) -> list[str]:
        return [c.value for c in cls]


class SintaxAsvResult(BaseModel):
    level: str
    tax_name: str
    score: float


class SintaxResult(BaseModel):
    result: dict[str, SintaxAsvResult]


def add_taxonomy(df: pd.DataFrame):
    tax = (
        df['reference']
        .str.split(';', expand=True)[1]
        .str.removeprefix('tax=')
        .str.split(',', expand=True)
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

    return pd.concat([df, tax], axis=1)


def parse_tsv(tsv: Path, threshold: float, outdir: Path):
    df = pd.read_csv(tsv, sep='\t', names=['asv', 'reference', 'num_hits', 'iteration'])

    df = add_taxonomy(df)

    # Remove residual accession from species name.
    df['species'] = df['species'].apply(lambda x: re.sub(pat, '', x))

    result_full = []
    for asv, asv_subset in df.groupby(by='asv'):
        asv_short: dict[str, SintaxAsvResult] = {}
        asv_full = []

        for level in Levels.as_list():
            (best_level, best_score) = (
                asv_subset[level]
                .value_counts()
                .reset_index()
                .sort_values(by='count', ascending=False)
                .iloc[0]
            )

            best_level = re.sub(r'(k|p|c|o|f|g|s):', '', best_level)

            asv_full.append(f'{best_level}({float(best_score)})')

            if asv not in asv_short and best_score >= threshold:
                asv_short[asv] = SintaxAsvResult(level=level, tax_name=best_level, score=best_score)

        result_full.append([asv, '|'.join(asv_full)])

    # Write full results.
    result_df = pd.DataFrame(result_full, columns=['asv', 'scores'])
    result_df.to_csv(outdir / 'result_full.tsv', sep='\t', index=False)

    # Write lowest valid taxonomic level per asv.
    with (outdir / 'result.json').open('w') as f:
        json.dump(SintaxResult(result=asv_short).model_dump(), f, indent=4)


def main():
    parser = argparse.ArgumentParser('Parse sintax tsv output from sintax_rs')
    parser.add_argument('--tsv', help='Path to sintax tsv file.', required=True)
    parser.add_argument(
        '--threshold',
        help='Cutoff for determining taxonomic level classification',
        required=False,
        type=float,
        default=0.80,
    )
    parser.add_argument('-o', '--outdir', help='Output directory', required=True)
    args = parser.parse_args()

    if not (tsv := Path(args.tsv)).is_file():
        raise FileNotFoundError(tsv)

    if (threshold := args.threshold) > 1.0:
        msg = f'Threshold {threshold} must be <= 1.0.'
        raise ValueError(msg)

    outdir = Path(args.outdir)
    outdir.mkdir(exist_ok=True, parents=True)

    parse_tsv(tsv, threshold, outdir)


if __name__ == '__main__':
    main()
