from three_geo import Point, Pyramid

# Point class
new_point = Point(1, 2, 3.4)

print(new_point)

print("X: " + str(new_point.x), 
      "Y: " + str(new_point.y), 
      "Z: " + str(new_point.z))


# Pyramid test
new_pyr = Pyramid(5, 6)

print("Height: ", new_pyr.height)
print("Base Length: ", new_pyr.base_length)
print("Apex:  X: ", new_pyr.apex.x, " Y: ", new_pyr.apex.y, " Z: ", new_pyr.apex.z)
print("Surface Area: ", new_pyr.surface_area())
for p in new_pyr.base:
      print("X: " + str(p.x), "Y: " + str(p.y), "Z: " + str(p.z))
