# dcp-curve

Утилита командной строки dcp-curve.exe

### Зачем?

Начал изучать Rust. После изучения начал и синтаксиса решил, не без помощи ИИ, сразу сделать полезность. Эта утилита строит график тоновой кривой для профиля dcp (DNG color profile), используемого инструментами Adobe для обработки raw файлов цифровых камер. Для корректной работы требуется "декомпилировать" профиль при помощи утилиты dcpTool. Цель - окунуться в Rust и получить наглядное представление о форме кривых, заложенных в профили и не более.

`dcptool -d "Sony ILCE-7 Camera Standard.dcp" "Sony ILCE-7 Camera Standard.xml"`

затем вызвать dcp-curve

`dcp-curve "Sony ILCE-7 Camera Standard.xml"`

будет получен jpeg размером 4096 * 2160 и именем  `Sony ILCE-7 Camera Standard.xml.curve.jpg`

### Как установить

* Стянуть к себе **dcp-curve.exe** из последнего [release](https://github.com/AndyAVS/dcp-curve/releases/)
* Положить рядом с dcpTool 

### Updates
* 16.12.2025 релиз 
