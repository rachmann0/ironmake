# gcc -E main.c -o main.i # Preprocessor -> C Program simplified basically
# gcc -E math.c -o math.i # Preprocessor -> C Program simplified basically

# gcc -S main.i -o main.s # Compilation -> Assembly file
# gcc -S math.i -o math.s # Compilation -> Assembly file

# gcc -c main.s -o main.o # Assembler -> Object file
# gcc -c math.s -o math.o # Assembler -> Object file

# gcc -c main.c -o main.o # Assembler -> Object file
# gcc -c math.c -o math.o # Assembler -> Object file




#? Direct Linking
# gcc -c main.c -o main.o
# gcc -c math.c -o math.o

# gcc main.o math.o -o myprogram # Link Obj together

#? Static Linking
# gcc -c main.c -o main.o
# gcc -c math.c -o math.o

# ar rcs libmath.a math.o # Create Static Library that is later Combined with entry

# gcc  main.o -o myprogram -lmath -L. # Link library

#? Dynamic Linking
gcc -c main.c -o main.o 
gcc -fPIC -c math.c -o math.o # Compile to Position Independant Code (PIC)

gcc -shared math.o -o libmath.so # Create Dynamic Library

gcc  main.o -o myprogram -lmath -L. # Link library
# gcc main.o -o myprogram -L. -lmath -Wl,-rpath=. # you can also embed the path into the exe

# LD_LIBRARY_PATH=./ ./myprogram 
# LD_LIBRARY_PATH=./:./lib ./myprogram  # (:) used as separator for multiple lib paths

# real projects typically dont use LD_LIBRARY_PATH
# real projects install lib system wide



#! archive (.a) (literally just a collection of .o)
# ar rcs libfinal.a lib1.o lib2.o lib3.o ... libn.o
#* can add more .o to existing .a
# ar rcs libfinal.a libx.o
#* can see the contents of archive (ar t)
# ar t libfinal.a
#* if you wannaa combine two .a extract both first
# ar x libfinal.a

#! linking
# gcc main.o \
#   -L./build \
#   -L./external \
#   -lutils \
#   -lmath \
#   -o myprogram