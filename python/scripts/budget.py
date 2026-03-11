class Category:

    def __init__(self, name: str):
        self.ledger = []
        self.name = name

    def __str__(self):
        title = f"*{self.name}*"
        length = len(title)
        leftover = round((30 - length) / 2)
        bars = "*" * leftover
        representation = ""
        representation += f"{bars}{title}{bars}\n"
        lines = []
        for entry in self.ledger:
            description = entry["description"][:23]
            amount = re.search(r"-\d+\.\d{2}$", str(entry["amount"]))
            if amount:
                amount = amount.group(0)[:7]
            else:
                amount = re.search(r"\-?\d+", str(entry["amount"]))
                amount = f"{amount.group(0)}.00"
            fill = (30 - len(description) - len(amount)) * " "
            lines.append(f"{description}{fill}{amount}")
        representation += "\n".join(lines)
        bal = self.get_balance()
        representation += f"\nTotal: {bal}"
        return representation

    def deposit(self, amount, description=""):
        if amount >= 0:
            self.ledger.append({"amount": amount, "description": description})

    def withdraw(self, amount, description=""):
        if amount >= 0:
            if self.check_funds(amount):
                self.ledger.append({"amount": -1 * amount, "description": description})
            else:
                return False
        else:
            return False
        return True

    def get_balance(self):
        bal = 0
        for transaction in self.ledger:
            bal += transaction["amount"]
        return bal

    def transfer(self, amount, target_category):
        if self.check_funds(amount):
            self.withdraw(
                amount=amount, description=f"Transfer to {target_category.name}"
            )
            target_category.deposit(amount, description=f"Transfer from {self.name}")
            return True
        return False

    def check_funds(self, amount):
        bal = self.get_balance()
        if amount > bal:
            return False
        return True


def create_spend_chart(categories):
    chart = "Percentage spect by category\n"
    totals = []
    for category in categories:
        total = 0
        for entry in category.ledger:
            if entry["amount"] < 0:
                total += abs(entry["amount"])
        totals.append(total)
    total_spent = sum(totals)
    percents = [int((x / total_spent) * 100) // 10 * 10 for x in totals]
    bars = len(percents)
    y_axis_prefixes = {100: "100| ", 0: "  0| "}
    for value in range(10, 100, 10):
        y_axis_prefixes.update({value: f" {value}| "})
    before_lines = []
    for value in reversed(range(0, 110, 10)):
        before_line = y_axis_prefixes[value]
        values = []
        for o in percents:
            if o >= value:
                values.append("o")
            else:
                values.append(" ")
        before_line += "  ".join(values)
        before_lines.append(before_line)
    footer = "\n    -"
    for category in categories:
        footer += "---"
    after_lines = []
    max_length = max([len(category.name) for category in categories])
    for i in range(max_length):
        values = ["   "]
        for category in categories:
            if len(category.name) > i:
                values.append(category.name[i])
            else:
                values.append(" ")
        after_line = "  ".join(values)
        after_lines.append(after_line)
    footer += "\n"
    chart += "\n".join(before_lines)
    chart += footer
    chart += "\n".join(after_lines)
    return chart
