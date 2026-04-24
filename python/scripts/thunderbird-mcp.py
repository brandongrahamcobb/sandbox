from database import CustomDatabase
from mcp.server.fastmcp import FastMCP

mcp = FastMCP("nutrition")


@mcp.tool()
async def create_nutrients_entry_into_database(
    food_name=None,
    serving_size_g=None,
    calories_kcal=None,
    protein_g=None,
    fat_g=None,
    carbohydrates_g=None,
    fiber_g=None,
    sugar_g=None,
    water_g=None,
    vitamin_a_mcg=None,
    vitamin_b1_mg=None,
    vitamin_b2_mg=None,
    vitamin_b3_mg=None,
    vitamin_b5_mg=None,
    vitamin_b6_mg=None,
    vitamin_b7_mcg=None,
    vitamin_b9_mcg=None,
    vitamin_b12_mcg=None,
    vitamin_c_mg=None,
    vitamin_d_mcg=None,
    vitamin_e_mg=None,
    vitamin_k_mcg=None,
    calcium_mg=None,
    iron_mg=None,
    magnesium_mg=None,
    phosphorus_mg=None,
    potassium_mg=None,
    sodium_mg=None,
    zinc_mg=None,
    copper_mg=None,
    manganese_mg=None,
    selenium_mcg=None,
    iodine_mcg=None,
    fluoride_mg=None,
    choline_mg=None,
    omega_3_g=None,
    omega_6_g=None,
    trans_fat_g=None,
    saturated_fat_g=None,
    monounsaturated_fat_g=None,
    polyunsaturated_fat_g=None,
) -> str:
    l = locals()
    values = [value for value in l.values() if value is not None]
    keys = [key for key, value in l.items() if value is not None]
    rset = range(1, len(l) + 1)
    async with CustomDatabase() as pool:
        async with pool.acquire() as conn:
            query = f"""
                INSERT INTO nutrients ({", ".join(keys)})
                VALUES ({", ".join([f"${n}" for n in rset])})
            """
            await conn.execute(query, *values)
    return "success!"


def main():
    mcp.run(transport="stdio")


if __name__ == "__main__":
    main()
