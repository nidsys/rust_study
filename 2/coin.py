#500원 10개, 100원 3대, 50원 10개 있을때 잔돈으로 3950원을 거슬러 줘야 할 경우 올 수 있는 모든 조합나열
price = 3950

for i500 in range(0,11):
  for i100 in range(0,4):
    for i50 in range(0,11):
      tot = (i50*50)+(i100*100)+(i500*500)

      if tot == price:
        print("500원x{}개, 100원x{}개, 50원x{}개 = {}".format(i500,i100,i50,tot))
