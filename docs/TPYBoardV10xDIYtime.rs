[Micropython]TPYBoard DIY����ʱ��
==================================

ԭ����Ȩ��ɽ���ܲ��Ƽ����޹�˾����,ת�ر�����������ʽע�����ߺ�ԭʼ������

ʵ��Ŀ��
----------------------
1. ѧϰ��PC��ϵͳ����չ��I/O �ӿڵķ�����
2. ʲô��SPI�ӿڡ�
3. ѧϰTPYBoard I2C�ӿڵ��÷���
4. ѧϰLCD5110���߷�����
5. �趨ʱ�Ӳ�����ǰʱ����ʾ��LCD5110�ϡ�

����Ԫ����
---------------------
DS3231ģ��һ��
TPYBoard����һ��
LCD5110��ʾ��һ��
������һ��
�Ű�������

TPYBoard��SPI�ӿ�
-------------------------------
LCD5110��ҪSPI�ӿ���TPYBoard�������Ӵ������ݣ�SPI�ӿ�����CPU����Χ��������֮�����ͬ���������ݴ��䣬TPYBoard������SPI�ӿڣ������õ�ΪSPI1�ӿڡ�

.. image:: http://www.tpyboard.com/ueditor/php/upload/image/20161126/1480150621179364.png

    TPYBoard��I2C�ӿ�
----------------------------------
DS3231��I2C�ӿ�ͨ�ŵģ���ͨ��I2C�ӿ���TPYboard��������ͨѶ��DS3231ͨ������ӿ���TPYBoard����˫��ͨѶ���������ݴ��䣬TPYBoard������I2C�ӿڣ����������õ���I2C�ӿ�1��

 .. image:: http://www.tpyboard.com/ueditor/php/upload/image/20161126/1480150651145892.png .. image:: http://www.tpyboard.com/ueditor/php/upload/image/20161126/1480150657750799.png

DS3231�Ľ��߷���
---------------------------------
DS����������ֻ�õ�DS3231��SCL,SDA,VCC,GND�ĸ���ż����趨������ǰʱ�䣬�����õ�λI2C�ӿ�1����DS3231�����SCL��TPYboard�����X9�����SDA��TPYBoard�����X10��

.. image:: http://www.tpyboard.com/ueditor/php/upload/image/20161126/1480150682569300.png

����5110��ʾ����ʾ6x8�ַ�
----------------------------------------
�ȿ�һ��LCD5110��ź���ɣ�ע�⣺LCD5110�������Щ��һ���ģ�

TPYBoard�������5110����Ŷ�Ӧ��ϵ���£�
------------------------------------------------------------
TPYBoard  LCD5110 memo

# any  Pin => RST  Reset pin (0=reset, 1=normal)
# any  Pin => CE Chip Enable (0=listen for input, 1=ignore input)
# any  Pin => DC Data/Command (0=commands, 1=data)
# MOSI => DIN  data flow (Master out, Slave in)
# SCK  => CLK  SPI clock
# 3V3  or any Pin => VCC  3.3V logic voltage (0=off, 1=on)
# any  Pin => LIGHT Light (0=on, 1=off)
# GND => GND

���ǿ������׵Ļ���ֱ������ű�Ű�

TPYBoard  LCD5110 memo

Y10 => RST  Reset pin (0=reset, 1=normal)
Y11    => CE    Chip Enable (0=listen for input, 1=ignore input)
Y9 => DC    Data/Command (0=commands, 1=data)
X8 => DIN   data flow (Master out, Slave in)
X6 => CLK   SPI clock
VCC
Y12    => LIGHT Light (0=on, 1=off)
GND

.. image:: http://www.tpyboard.com/ueditor/php/upload/image/20161126/1480150739943842.png

Դ����
-------------------
����ok�󣬲��ҵ���font.py�ļ���upcd8544.py�ļ��Լ�DS3231.py����дmain.py�趨ʱ�䣬����main.py���ɽ���ǰʱ����ʾ��5110��ʾ���ϡ�
1  # main.py -- put your code here!
2  import pyb
3  import upcd8544
4  from machine import SPI,Pin
5  from DS3231 import DS3231
6    
7  ds=DS3231(1)
8  ds.DATE([16,11,26])
9  ds.TIME([16,14,6])
10    
11 while True:
12     ds.TEMP()
13     ds.DATE()
14     SPI = pyb.SPI(1) #DIN=>X8-MOSI/CLK=>X6-SCK
15     #DIN =>SPI(1).MOSI 'X8' data flow (Master out, Slave in)
16     #CLK =>SPI(1).SCK  'X6' SPI clock
17         RST    = pyb.Pin('Y10')
18         CE     = pyb.Pin('Y11')
19         DC     = pyb.Pin('Y9')
20         LIGHT  = pyb.Pin('Y12')
21     lcd_5110 = upcd8544.PCD8544(SPI, RST, CE, DC, LIGHT)
22     lcd_5110.lcd_write_string('Date',0,0)
23     lcd_5110.lcd_write_string(str(ds.DATE()),0,1)
24     lcd_5110.lcd_write_string('Time',0,2)
25     lcd_5110.lcd_write_string(str(ds.TIME()),0,3)
26     lcd_5110.lcd_write_string('Tem',0,4 )
27     lcd_5110.lcd_write_string(str(ds.TEMP()),0,5)
28     pyb.delay(1000)

