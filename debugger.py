import json
def parse_our():
    data = {}
    rows = []
    for i in open("outputs/impl.txt", "r").read().split("\n"):
        if len(i) == 0:
            rows.append(data)
            data = {}
        else:
            i = list(map(lambda x: x.split(" "), i.split(":")))
            i = list(filter(lambda x: len(x), sum(i, [])))
            for index in range(0, len(i) - 1, 2):
                data[i[index]] = i[index + 1]
    return rows
    
def parse_truth():
    rows = []
    for i in json.loads(open("outputs/truth.txt", "r").read()):
        lines = i.split("\n")
        register = lines[0] + " " + lines[1]
        status = lines[3]

        register = list(map(lambda x: x.split(" "), register.split("=")))
        register = list(filter(lambda x: len(x), sum(register, [])))
        data = {}
        for index in range(0, len(register) - 1, 2):
            data[register[index]] = register[index + 1]

        rows.append(data)
    return rows

rows_our = parse_our()
rows_truth = parse_truth()

prev_pc = None
for i in range(min(len(rows_our), len(rows_truth))):
    row_our = rows_our[i]
    row_truth = rows_truth[i]

    our_pc = int(row_our["pc"], 16)
    truth_pc = int(row_truth["PC"].replace("$", ""), 16) - 0x0600

    our_sp = int(row_our["sp"], 16)
    truth_sp = int(row_truth["SP"].replace("$", ""), 16)

  #  if (our_sp != truth_sp):
   #     print(hex(prev_pc))
    #    print(our_sp, truth_sp)
     #   raise Exception("Misaligned stack")

    if (our_pc != truth_pc):
        print(hex(our_pc), hex(truth_pc))
        print(hex(prev_pc))
        print("index ", i)
        raise Exception("Misaligned pc")
    prev_pc = our_pc
