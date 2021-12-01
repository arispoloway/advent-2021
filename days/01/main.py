from collections import defaultdict, Counter

def main(lines):
  for line in lines:
    print(line)

if __name__ == '__main__':
  with open('in.txt') as f:
    main(map(lambda x: x.rstrip('\n'), f.readlines()))

