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
