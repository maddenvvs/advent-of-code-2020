from typing import TextIO

import click

from .solutions import all_solutions

CONTEXT_SETTINGS = dict(
    help_option_names=['-h', '--help'])


@click.command(context_settings=CONTEXT_SETTINGS)
@click.argument("day", type=click.IntRange(min=1, max=25))
@click.argument("file", type=click.File("r"))
def cli(day: int, file: TextIO):
    """AoC 2020 CLI application.

    Display solutions for DAY with problem input containing in FILE.
    """

    if day > len(all_solutions):
        click.echo(f"There is no solution for day {day} yet. Stay tuned!")
        return

    file_content = file.read()
    solution = all_solutions[day - 1]()

    click.echo(f"Day {day}-1: {solution.first_task(file_content)}")
    click.echo(f"Day {day}-2: {solution.second_task(file_content)}")
