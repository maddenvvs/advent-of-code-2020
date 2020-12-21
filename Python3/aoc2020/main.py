from typing import TextIO

import sys
import click

from .solutions import get_solution

CONTEXT_SETTINGS = dict(
    help_option_names=['-h', '--help'])


@click.command(context_settings=CONTEXT_SETTINGS)
@click.argument("day", type=click.IntRange(min=1, max=25))
@click.argument("file", type=click.File("r"))
def cli(day: int, file: TextIO):
    """AoC 2020 CLI application.

    Display solutions for DAY with problem input containing in FILE.
    """

    solution = get_solution(day)

    if solution is None:
        click.echo(f"There is no solution for day {day} yet. Stay tuned!")
        return

    file_content = file.read()

    click.echo(f"Day {day}-1: {solution.first_task(file_content)}")
    click.echo(f"Day {day}-2: {solution.second_task(file_content)}")
