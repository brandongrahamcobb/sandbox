import pytest

class Employee:
    
    _base_salaries = {
        'trainee': 1000,
        'junior': 2000,
        'mid-level': 3000,
        'senior': 4000
    }
    def __init__(self, name, level):
        if not isinstance(name, str) or not isinstance(level, str):
            raise TypeError("'name' and 'level' attribute must be of type 'str'.")
        if level not in Employee._base_salaries.keys():
            raise ValueError(f"Invalid value '{level}' for 'level' attribute.")
        self._name = name
        self._level = level
        self._salary = Employee._base_salaries[level]

    def __str__(self):
        return f"{self.name}: {self.level}"
    
    @property
    def name(self):
        return self._name
    
    @name.setter
    def name(self, new_name):
        if not isinstance(new_name, str):
            raise TypeError("'name' must be a string.")
        self._name = new_name
        print(f"'name' updated to '{new_name}'.")
    
    @property
    def level(self):
        return self._level
    
    @level.setter
    def level(self, new_level):
        if new_level not in Employee._base_salaries.keys():
           raise ValueError(f"Invalid value '{new_level}' for 'level' attribute.")
        if new_level == self._level:
            raise ValueError(f"'{self._level}' is already the selected level.")
        if Employee._base_salaries[new_level] < Employee._base_salaries[self._level]:
            raise ValueError(f"Cannot change to lower level.")
        print(f"'{self._name}' promoted to '{new_level}'.")
        self.salary = Employee._base_salaries[new_level]
        self._level = new_level
    
    @property
    def salary(self):
        return self._salary
    
    @salary.setter
    def salary(self, new_salary):
        if not isinstance(new_salary, int) and not isinstance(new_salary, float):
            raise TypeError(f"'salary' must be a number.")
        if new_salary < self._salary:
            raise ValueError(f"Salary must be higher than minimum salary ${self._salary}.")
        self._salary = new_salary
        print(f"Salary updated to ${new_salary}.")

    
    def __repr__(self):
        return f"Employee('{self.name}', '{self.level}')"
    
    @property
    def salary(self):
        return self._salary
    
charlie_brown = Employee("Charlie Brown", "trainee")
print(f"Base salary: ${charlie_brown.salary}")
@pytest.mark.parametrize(
    "name, level",
    [
        ("Charlie Brown", "trainee")
    ]
)
def test_employee(name, level):
    employee = Employee(name=name, level=level)
    assert employee.name == name
    assert employee.level == level
    assert repr(employee) == f"Employee('{name}', '{level}')"