import bpy
print("\n\n\n\n")
def printit(vertices, a, b):
    output = "t,"
    for vertex in vertices:
        index = vertex 
        #print(a[index].co)
        fdsa = b@a[index].co
        for direction_index in range(len(fdsa)):
            output += str(fdsa[direction_index])
            output += ","
            
            
    output += "1,1,1"
    with open("/home/natalie/Documents/cgraphics/blender/test.csv", "a") as file:
        file.write(output)
        file.write("\n")

with open("/home/natalie/Documents/cgraphics/blender/test.csv", "w") as file:
    file.write("l,-11,0,0,1\n")

for mesh in bpy.data.meshes:
    matrix = 0
    for object in bpy.data.objects:
        if object.name == mesh.name:
            matrix = object.matrix_world
            print(matrix)
            
    
    if len(mesh.vertices) == 8:
        continue
    
    vertices = mesh.vertices
    
    print(vertices)
    for polygon in mesh.polygons:
        if len(polygon.vertices) == 4:
            printit(polygon.vertices[:3], vertices, matrix)
            printit(polygon.vertices[1:], vertices, matrix)
        else:
            printit(polygon.vertices, vertices, matrix)