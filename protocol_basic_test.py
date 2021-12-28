import socket, time

time.sleep(1)

s1 = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s2 = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s1.connect(("localhost", 8080))
s2.connect(("localhost", 8080))

# crea la queue 'Messi'
define_test = [0x64, 0, 5, 77, 101, 115, 115, 105]

# pushea el mensaje 'hola messi' a la queue 'Messi'
push_test = [0x75, 0, 5, 77, 101, 115, 115, 105, 0, 10, 104, 111, 108, 97, 32, 109, 101, 115, 115, 105]

# pushea el mensaje 'hola messii' a la queue 'Messi'
push_test_2 = [0x75, 0, 5, 77, 101, 115, 115, 105, 0, 11, 104, 111, 108, 97, 32, 109, 101, 115, 115, 105, 105]

# popea el ultimo mensaje de la queue 'Messi'
pop_test = [0x6f, 0, 5, 77, 101, 115, 115, 105]

x1 = s1.sendall(bytes(define_test))
time.sleep(1)
x2 = s2.sendall(bytes(push_test))
time.sleep(1)
x3 = s1.sendall(bytes(push_test_2))
time.sleep(1)

print(x1, x2, x3)
