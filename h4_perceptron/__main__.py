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
from sys import stdout

fig = plt.figure()
ax = fig.add_subplot(111)
ax.set_xlim([-10, 10])
ax.set_ylim([-10, 10])

dots1 = []
dots2 = []

def on_click(event):
    print('button=%d, x=%d, y=%d, xdata=%f, ydata=%f' %
          (event.button, event.x, event.y, event.xdata, event.ydata))
    if event.button == 1:
        plt.plot(event.xdata, event.ydata, 'bo')
        dots1.append([event.xdata, event.ydata])
    else:
        plt.plot(event.xdata, event.ydata, 'rx')
        dots2.append([event.xdata, event.ydata])
    fig.canvas.draw()

def on_press(event):
    print('key=%s' % event.key)
    stdout.flush()
    if event.key == 'c':
        plt.clf()
        fig.canvas.mpl_disconnect(on_click)
        fig.canvas.mpl_disconnect(on_press)
        wait_time = input("How long are you willing to wait? Enter the number of minutes...")


cid_click = fig.canvas.mpl_connect('button_press_event', on_click)
cid_press = fig.canvas.mpl_connect('key_press_event', on_press)

plt.show()
