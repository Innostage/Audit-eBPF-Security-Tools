from cProfile import label
import matplotlib.pyplot as plt

# your files with registered events
filenames = [
    "tetragon_speed_1k_prc.txt",
    "tetragon_speed_1k_2_prc.txt",
    "tetragon_speed_1k_3_prc.txt",
]

dots = []

for filename in filenames:
    l_dots = []
    with open(filename) as f:
        for line in f:
            # getting arg as number of event ('/bin/cat 1')
            l_dots.append(int(line.rstrip().rsplit(" ", 1).pop()))
    dots.append(l_dots)

plt.figure(facecolor="black")
ax = plt.axes()
ax.set_facecolor("black")

ax.spines["bottom"].set_color("white")
ax.spines["top"].set_color("white")
ax.spines["right"].set_color("white")
ax.spines["left"].set_color("white")

ax.tick_params(axis="x", colors="white", labelsize=12)

for i, l_dots in enumerate(dots):
    y = [i * 0.5] * len(l_dots)
    plt.plot(
        l_dots,
        y,
        color="green",
        linestyle="solid",
        linewidth=1,
        marker="|",
        markerfacecolor="green",
        markersize=5,
    )

plt.ylim(-0.5, len(dots) * 0.5)
plt.xlim(1, 8000)
plt.xlabel("Events", color="white", size=14)
plt.ylabel("Runs", color="white", size=14)

# edit your title
plt.title("Speed execution on Tetragon", color="white")
plt.show()
