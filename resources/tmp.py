import os
import subprocess

def main():
    files = [x for x in os.listdir() if x.endswith(".png")]
    files.sort()
    for file in files:
        print(f"{file}                   ", end='\r')
        subprocess.run([
            "convert",
            file,
            "-alpha",
            "on",
            file
        ])
    return None

if __name__ == "__main__":
    main()
