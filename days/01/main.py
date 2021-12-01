from collections import defaultdict, Counter

def main(lines):
  last = 0
  count = -1
  for line in lines:
    new = int(line)
    if new > last:
      count += 1
    last = new
  print(count)

  last = 0
  count = -1
  for i in range(len(lines) - 2):
    new = sum(map(int, lines[i:i+3]))
    if new > last:
      count += 1
    last = new
  print(count)

if __name__ == '__main__':
  with open('in.txt') as f:
    main(list(map(lambda x: x.rstrip('\n'), f.readlines())))

