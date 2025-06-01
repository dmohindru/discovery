# I2C

We just saw the serial communication protocol. It's a widely used protocol because it's very
simple and this simplicity makes it easy to implement on top of other protocols like Bluetooth and
USB.

However, it's simplicity is also a downside. More elaborated data exchanges, like reading a digital
sensor, would require the sensor vendor to come up with another protocol on top of it.

(Un)Luckily for us, there are _plenty_ of other communication protocols in the embedded space. Some
of them are widely used in digital sensors.

The F3 board we are using has three motion sensors in it: an accelerometer, a magnetometer and
gyroscope. The accelerometer and magnetometer are packaged in a single component and can be accessed
via an I2C bus.

I2C stands for Inter-Integrated Circuit and is a _synchronous_ _serial_ communication protocol. It
uses two lines to exchange data: a data line (SDA) and a clock line (SCL). Because a clock line is
used to synchronize the communication, this is a _synchronous_ protocol.

<p align="center">
<img class="white_bg" height=180 title="I2C bus" src="https://upload.wikimedia.org/wikipedia/commons/3/3e/I2C.svg">
</p>

This protocol uses a _master_ _slave_ model where the master is the device that _starts_ and
drives the communication with a slave device. Several devices, both masters and slaves, can be
connected to the same bus at the same time. A master device can communicate with a specific slave
device by first broadcasting its _address_ to the bus. This address can be 7 bits or 10 bits long.
Once a master has _started_ a communication with a slave, no other device can make use of the bus
until the master _stops_ the communication.

The clock line determines how fast data can be exchanged and it usually operates at a frequency of
100 KHz (standard mode) or 400 KHz (fast mode).

Code fix
https://github.com/rust-embedded/discovery/issues/274
