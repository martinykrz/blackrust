import os
import subprocess

def main():
    files = [x for x in os.listdir() if x[:1] == 'a']
    files.sort()
    # for file in files:
    #     subprocess.run([
    #         "mv",
    #         file,
    #         f"0{file}"
    #     ])
    print(files)
    return None

if __name__ == "__main__":
    main()
