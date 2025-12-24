import time
import sys

print("Python process started!!!!", flush=True)

while True:
    try:
        print("Enter something: ", flush=True)
        user_input = input()
        if user_input == "q":
            print("Quitting", flush=True)
            break
        print(f"You entered: {user_input}", flush=True)
        print(f"Processing... {time.time()}", flush=True)
    except EOFError:
        break
    except KeyboardInterrupt:
        break

print("Python process ending", flush=True)
