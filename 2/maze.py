import random

MAP_N = 25
maze =  [] # 리스트 변수

#초기화
for y in range(0, MAP_N):
  maze.append([0 for x in range(0, MAP_N)])

#벽
for n in range(0, MAP_N):
  maze[n][0] = maze[n][MAP_N-1] = 1 #좌 우
  maze[0][n] = maze[MAP_N-1][n] = 1 #아래 위

#2 마다 1개 벽
for y in range(2, MAP_N-2): #벽이 있으니 -2 적용
  for x in range(2, MAP_N-2):
    if x % 2 ==1 or y % 2 == 1: continue

    maze[y][x] = 1

    #상하좌우 중 어느 하나를 벽으로 마들기
    r = random.randint(0, 3)

    if r == 0: maze[y-1][x] = 1 #상
    if r == 1: maze[y+1][x] = 1 #하
    if r == 2: maze[y][x-1] = 1 #좌
    if r == 3: maze[y][x+1] = 1 #우

#출력
#0과 1을 각 흰색 타일(U+2B1C)과 검은색 타일(U+2B1B)로 치환한다
titles = ['☐','◼︎']

for y in range(0, MAP_N):
  for x in range(0, MAP_N):
    print(titles[maze[y][x]], end="")
  print("")
        
