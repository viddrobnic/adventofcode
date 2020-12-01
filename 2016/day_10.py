from collections import defaultdict
import re

commands = []

while True:
    try:
        commands.append(input())
    except:
        break

bots = {}
outputs = {}

class Bot:
    number = ''

    value_1 = -1
    value_2 = -1

    low_to = ''
    low_to_output = False

    high_to = ''
    high_to_outupt = False

    def give_value(self, value):
        if self.value_1 == -1:
            self.value_1 = value
        elif self.value_2 == -1:
            self.value_2 = value

    def excecute(self):
        if self.value_1 == -1 or self.value_2 == -1:
            return

        if min(self.value_1, self.value_2) == 17 and max(self.value_1, self.value_2) == 61:
            print('#1:', self.number)

        if not self.low_to_output:
            bots[self.low_to].give_value(min(self.value_1, self.value_2))
            bots[self.low_to].excecute()
        else:
            outputs[self.low_to][min(self.value_1, self.value_2)] += 1

        if not self.high_to_outupt:
            bots[self.high_to].give_value(max(self.value_1, self.value_2))
            bots[self.high_to].excecute()
        else:
            outputs[self.high_to][max(self.value_1, self.value_2)] += 1

        self.value_1 = -1
        self.value_2 = -1

for command in commands:
    match = re.findall(r'output (\d+)', command)
    for m in match:
        outputs[m] = defaultdict(int)

    match = re.findall(r'bot (\d+)', command)
    for m in match:
        bot = Bot()
        bot.number = m
        bots[m] = bot

for command in commands:
    if command[:3] == 'bot':
        low_to = re.search(r'(\d+)', command[command.find('low'): command.find('high')]).group(1)
        high_to = re.search(r'(\d+)', command[command.find('high'):]).group(1)

        low_to_output = command[command.find('low'): command.find('high')].find('output') != -1
        high_to_outupt = command[command.find('high'):].find('output') != -1

        bot_num = re.match(r'bot (\d+)', command).group(1)
        bot = bots[bot_num]
        bot.low_to = low_to
        bot.high_to = high_to
        bot.low_to_output = low_to_output
        bot.high_to_outupt = high_to_outupt

for command in commands:
    if command[:5] == 'value':
        match = re.match(r'value (\d+).*bot (\d+)', command)
        value = int(match.group(1))
        bot_num = match.group(2)

        bot = bots[bot_num]
        bot.give_value(value)

for bot in bots.values():
    bot.excecute()

print('#2:', int(list(outputs['0'].keys())[0]) * int(list(outputs['1'].keys())[0]) * int(list(outputs['2'].keys())[0]))
