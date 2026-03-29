def modify_value(mut memory_ref: Int):
    memory_ref = memory_ref * 2

def main():

    comptime multiplier_type = Int
    
    var x: multiplier_type = 10
    
    print("Original value of x:", x)
    
    modify_value(x)
    
    print("Modified value of x via mut reference:", x)