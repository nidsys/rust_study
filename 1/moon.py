# 지구에서 달까지의 거리는 384400km이다. 지구에서 달까지 80km/h 속도의 자동차와 300km/h 속도의 KTX 로 간다면 각 이동 수단은 며칠이 걸리는지 계산하시오
moon = 388440
car = 80
ktx = 300

print("달까지 자동차로 {}일".format( moon / car / 24))
print("달까지 KTX로 {}일".format(moon / ktx / 24))