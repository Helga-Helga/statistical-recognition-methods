# MIT License
#
# Copyright (c) 2018 Olga Laviagina
#
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.
import matplotlib.pyplot as plt
from matplotlib.animation import FuncAnimation
import perceptron
from sys import stdout
from numpy import linspace, zeros

fig = plt.figure()
ax = fig.add_subplot(111)
ax.set_xlim([-1, 1])
ax.set_ylim([-1, 1])

inside = []
outside = []
a = zeros(6)

def add_point(x, y, marker, array):
    ax.plot(x, y, marker)
    array.append([x, y])

def on_click(event):
    print('button=%d, x=%d, y=%d, xdata=%f, ydata=%f' %
          (event.button, event.x, event.y, event.xdata, event.ydata))
    if event.button == 1:
        add_point(event.xdata, event.ydata, 'bo', inside)
    if event.button == 3:
        add_point(event.xdata, event.ydata, 'ro', outside)

def animate(i):
    ax.clear()
    for x, y in inside:
        ax.plot(x, y, 'bo')
    for x, y in outside:
        ax.plot(x, y, 'ro')
    global a
    a = perceptron.train_on_dots(inside, outside, a)
    a = perceptron.train_on_eig(a)
    x = linspace(-1, 1, 100)
    y = linspace(-1, 1, 100)
    z = zeros((100, 100), dtype=float)
    for i in range(100):
        for j in range(100):
            z[j, i] = a.dot(perceptron.get_vector_in_new_space(x[i], y[j]))
    ax.contour(x, y, z, levels=[0])


cid_click = fig.canvas.mpl_connect('button_press_event', on_click)
anim = FuncAnimation(fig, animate, interval=50)
plt.show()
