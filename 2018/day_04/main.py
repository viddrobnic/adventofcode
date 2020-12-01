import sys
from datetime import datetime, timedelta
from collections import defaultdict


# Read the data
data = []
for i in sys.stdin:
    d, text = i.strip().split('] ', 1)
    d = d[1:]
    date = datetime.strptime(d, '%Y-%m-%d %H:%M')
    data.append((date, text))

data.sort(key=lambda x: x[0])

guard_sleep_total = defaultdict(int)
guard_sleep_minutes = {}

last_date = None
guard = None
for entry in data:
    date, text = entry
    if text == 'falls asleep':
        last_date = date
    elif text == 'wakes up':
        time_asleep = (date - last_date).seconds // 60
        guard_sleep_total[guard] += time_asleep

        while last_date < date:
            guard_sleep_minutes[guard][last_date.minute] += 1
            last_date += timedelta(minutes=1)
        last_date = date

    else:
        guard = int(text.split()[1][1:])
        if guard not in guard_sleep_minutes:
            guard_sleep_minutes[guard] = defaultdict(int)

# Part 1
max_guard, max_time = 0, 0
for guard in guard_sleep_total.keys():
    guard_sleep = guard_sleep_total[guard]
    if guard_sleep > max_time:
        max_guard, max_time = guard, guard_sleep


def minute_max_asleep(guard):
    minutes = guard_sleep_minutes[guard]
    max_minute, max_count = 0, 0

    for minute in minutes.keys():
        count = minutes[minute]
        if count > max_count:
            max_minute, max_count = minute, count

    return max_minute, max_count


max_minute, _ = minute_max_asleep(max_guard)
print(f'Part 1: {max_guard * max_minute}')

# Part 2
guard_max_asleep_minute = {}
for guard in guard_sleep_minutes.keys():
    guard_max_asleep_minute[guard] = minute_max_asleep(guard)

max_guard, max_minute, max_count = 0, 0, 0
for guard in guard_max_asleep_minute.keys():
    minute, count = guard_max_asleep_minute[guard]
    if count > max_count:
        max_guard, max_minute, max_count = guard, minute, count

print(f'Part 2: {max_guard * max_minute}')

