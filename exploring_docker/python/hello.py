import datetime

def main():
    now = datetime.datetime.now()
    print("Hello ASL!")
    print(now.strftime("%Y-%m-%d"))

if __name__ == "__main__":
    main()
