# dcp-curve

Утилита командной строки dcp-curve.exe

### Зачем?

Начал изучать rust. После изучения начал и синтаксиса решил, не без помощи ИИ, сразу сделать полезность. Эта утилита строит график тоновой кривой для профиля dcp (DNG color profile), используемого инструментами Adobe для обработки raw файлов цифровых камер. Для корректной работы требуется "декомпилировать" профиль при помощи утилиты dcpTool. Цель - окунуться с rust и получить наглядное представление о форме кривых, заложенных в профили и не более.

`dcptool -d "Sony ILCE-7 Camera Standard.dcp" "Sony ILCE-7 Camera Standard.xml"`

затем вызвать dcp-curve

`dcp-curve "Sony ILCE-7 Camera Standard.xml"`

будет получен jpeg размером 4096 * 2160 и именем  `Sony ILCE-7 Camera Standard.xml.curve.jpg`

### Как установить

* Стянуть к себе [release](https://github.com/AndyAVS/dcp-curve/releases/download/v0.2.0/dcp-curve.exe). 
* Положить рядом с dcpTool 

### Updates
* 16.12.2025 релиз 
