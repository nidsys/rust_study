import random

#1에서 75까지 리스트 등록
nums = list(range(1, 76))

#shuffle
random.shuffle(nums)

#가로 세로 칸수
N = 5

#가운데
nums[int(N*N/2)] = "*"

# i = 0;
# for y in range(0, N):
#   for x in range(0, N):
#     print("{:2}, ".format(nums[i]), end="")
#     i += 1
#   print("")

for y in range(0, N):
  for x in range(0, N):
    print("{:>3}, ".format(nums[y*N+x]), end="")
  print("")
