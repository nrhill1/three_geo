from three_geo import Point, Pyramid



###############################################################################
# POINT CLASS
###############################################################################

new_point = Point(1.0, 2, 3.4)

print(new_point)

print("X: " + str(new_point.x), 
      "Y: " + str(new_point.y), 
      "Z: " + str(new_point.z))

###############################################################################
# PYRAMID CLASS
###############################################################################
new_pyr = Pyramid(5.4, 6, -7.0, 0.8)

print("Height: ", new_pyr.height)
print("Base Length: ", new_pyr.base_length)
print("Apex:" +   
      " X: ", new_pyr.apex.x, 
      " Y: ", new_pyr.apex.y, 
      " Z: ", new_pyr.apex.z)
print("X offset: ", new_pyr.x_off, "Y offset: ", new_pyr.y_off)
print("Base Area: ", new_pyr.base_area())
print("Surface Area: ", new_pyr.surface_area())
print("Volume: ", new_pyr.volume())
print("Base Points: ")
for p in new_pyr.base:
      print("X: " + str(p.x), "Y: " + str(p.y), "Z: " + str(p.z))
