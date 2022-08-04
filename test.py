from three_geo import Point, Pyramid

new_point = Point(1, 2, 3.4)

print(new_point)

print("X: " + str(new_point.x), 
      "Y: " + str(new_point.y), 
      "Z: " + str(new_point.z))

new_pyr = Pyramid(5, 6)


print(new_pyr.base, new_pyr.height)