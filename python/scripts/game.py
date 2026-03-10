

class GameCharacter:

    def __init__(self, name):
        self._name = name
        self._health = 100
        self._mana = 50
        self._level = 1

    @property
    def name(self):
        return self._name
    
    @property
    def health(self):
        return self._health
    
    @health.setter
    def health(self, nh: int):
        if nh < 0:
            self._health = 0
        elif nh > 100:
            self._health = 100
        else:
            self._health = nh

    @property
    def mana(self):
        return self._mana
    
    @mana.setter
    def mana(self, nm: int):
        if nm < 0:
            self._mana = 0
        elif nm > 50:
            self._mana = 50
        else:
            self._mana = nm
    
    @property
    def level(self):
        return self._level
    
    def level_up(self):
        self._level += 1
        self.health = 100
        self.mana = 50
        print(f"{self.name} leveled up to {self._level}!")

    def __str__(self):
        return "\n".join([f"Name: {self.name}", f"Level: {self._level}", f"Health: {self.health}", f"Mana: {self.mana}"])

def test_stat():
    hero = GameCharacter('Kratos') # Creates a new character named Kratos
    print(hero)  # Displays the character's stats
    
    hero.health -= 30  # Decreases health by 30
    hero.mana -= 10    # Decreases mana by 10
    print(hero)  # Displays the updated stats
    
    hero.level_up()  # Levels up the character
    print(hero)