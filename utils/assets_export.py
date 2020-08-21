#!/usr/bin/env python3
"""Convert one `.svg` file to many `png`s."""

import subprocess
import os


EXPORT_IDS = [
    'tile',
    'tile_rocks',
    'poison_cloud',
    'imp',
    'imp_toxic',
    'imp_summoner',
    'imp_summoner_cast',
    'imp_bomber',
    'imp_bomber_throw',
    'grass',
    'shadow',
    'fire',
    'boulder',
    'bomb',
    'bomb_fire',
    'bomb_poison',
    'bomb_demonic',
    'explosion_ground_mark',
    'blood',
    'slash',
    'smash',
    'pierce',
    'claw',
    'spike_trap',
    'dot',
    'selection',
    'white_hex',
    'hammerman',
    'heavy_hammerman',
    'spearman',
    'elite_spearman',
    'heavy_spearman',
    'alchemist',
    'alchemist_throw',
    'alchemist_heal',
    'healer',
    'healer_throw',
    'healer_heal',
    'firer',
    'firer_throw',
    'swordsman',
    'elite_swordsman',
    'elite_swordsman_rage',
    'heavy_swordsman',
    'effect_poison',
    'effect_stun',
    'effect_bloodlust',
    'icon_ability_club',
    'icon_ability_knockback',
    'icon_ability_jump',
    'icon_ability_dash',
    'icon_ability_rage',
    'icon_ability_heal',
    'icon_ability_bomb_push',
    'icon_ability_bomb',
    'icon_ability_bomb_fire',
    'icon_ability_bomb_poison',
    'icon_ability_bomb_demonic',
    'icon_ability_summon',
    'icon_ability_bloodlust',
    'icon_info',
    'icon_end_turn',
    'icon_menu',
]
INPUT_FILE_NAME = os.path.join('assets_src', 'atlas.svg')
OUT_DIR_NAME = os.path.join('assets', 'img')


def _main() -> None:
    os.makedirs(OUT_DIR_NAME, exist_ok=True)
    for export_id in EXPORT_IDS:
        cmd = [
            'resvg',
            '--zoom=12',
            f'--export-id={export_id}',
            INPUT_FILE_NAME,
            os.path.join(OUT_DIR_NAME, f'{export_id}.png'),
        ]
        print(cmd)
        subprocess.run(cmd, check=True)


_main()
