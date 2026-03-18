# from database import CustomDatabase
# import pytest
# import sys
#
#
# @pytest.mark.asyncio
# async def test_plot():
#     assert await plot()
#
#
# async def plot():
#     pass
#
#
# @pytest.mark.asyncio
# async def test_main():
#     asyncio.run(main())
#
#
# # async def main():
# #
# #     option = sys.argv[1]
# #     async with CustomDatabase() as pool:
# #         async with pool.acquire() as conn:
# #             conn.fetch("""
# #                 SELECT * FROM moderation_logs
# #                 WHERE identifier = $1
# #             """, )
#
#
# class Plot(commands.Cog):
#
#     __options = ["ban", "tmute", "vmute"]
#
#     def __init__(self):
#         self.__option: str | None = None
#
#     @property
#     def option(self):
#         return self.__option
#
#     @option.setter
#     def option(self, new_option: str):
#         if new_option in self.__options:
#             self.__option = new_option
#         else:
#             raise ValueError(f"New option must be one of {", ".join(self.__options)}.")


import discord
import pandas as pd
import matplotlib.pyplot as plt
import io


class PlotView(discord.ui.View):

    def __init__(self, logger, pool):
        super().__init__()
        self.__logger = logger
        self.__pool = pool

    @discord.ui.select(
        options=[
            discord.SelectOption(label="ban"),
            discord.SelectOption(label="tmute"),
            discord.SelectOption(label="vmute"),
        ],
        placeholder="Select an option",
    )
    async def select(self, interaction: discord.Interaction, select) -> None:
        await interaction.response.defer()
        await self.plot(interaction, select.values[0])
        await interaction.followup.send(
            f"You selected {select.values[0]}", ephemeral=True
        )

    async def plot(self, interaction, option: str):
        async with self.__pool.acquire() as conn:
            rows = await conn.fetch(
                """
                SELECT * FROM moderation_logs
                WHERE infraction_type=$1 OR infraction_type=$2
                ORDER BY created_at ASC
            """,
                option,
                f"un{option}",
            )
        data = {}
        enforce = []
        undo = []
        for row in rows:
            if row["infraction_type"] == option:
                enforce.append(row)
                data[row["created_at"]] = option
            elif row["infraction_type"] == f"un{option}":
                undo.append(row)
                data[row["created_at"]] = f"un{option}"
        for enforce_item in enforce:
            for undo_item in undo:
                if (
                    undo_item["channel_snowflake"] == enforce_item["channel_snowflake"]
                    and undo_item["target_member_snowflake"]
                    == enforce_item["target_member_snowflake"]
                ):
                    break
            else:
                if enforce_item["expires_at"] is not None:
                    data[enforce_item["expires_at"]] = f"un{option}"
        total = 0
        totals = {}
        for k, v in sorted(data.items()):
            if v == f"un{option}":
                total -= 1
            elif v == option:
                total += 1
            totals[k] = total
        df = pd.DataFrame.from_dict(totals, orient="index", columns=["Total"])
        df.plot()
        picture = io.BytesIO()
        plt.title(f"Accumulation of Active {option}s over Time")
        plt.xlabel("Date")
        plt.ylabel(f"Active {option}s")
        plt.savefig(picture, format="png")
        picture.seek(0)
        await interaction.followup.send(
            file=discord.File(fp=picture, filename="image.png")
        )
