import sys

def area_of_circle(r: float) -> float:
    return 3.14 * r * r

radius = float(sys.argv[1])
print(area_of_circle(radius))