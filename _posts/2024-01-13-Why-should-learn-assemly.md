---
title:  "Tại sao kỹ sư hệ thống nhúng nên học ngôn ngữ assembly?"
categories:
   - Engineering
   - Vietnamese
tags:
---
**Info Notice:** Đây là bài viết đầu tiên tôi viết bằng tiếng Việt với mong muốn bài viết này đến được với nhiều đọc giả còn đang là sinh viên hoặc những kỹ sư trẻ đang làm ngành phần mềm nhúng. Bài viết này dựa trên bài viết [EABI - Why any embedded software engineer should know about it.](https://rtos.dev/engineering/EABI/).
{: .notice--info}


# Tại sao kỹ sưu hệ thống nhúng nên học assembly?
Mình vốn không học ngành khoa học máy tính hay những ngành liên quan, nhưng may mắn mình được học khóa kiến trúc máy tính và hợp ngữ khi còn học đại học, và may mắn thay tôi được học với một người thầy lớn tuổi. Có lẽ vì thầy lớn tuổi nên thầy đã trả qua một thời kỳ khi mà các ngôn ngữ cấp cao chưa mạnh, hay các MCU tại thời điểm thầy làm việc có bộ nhớ khá khiêm tốn, và thực sự thì các ứng dụng sử dụng MCU tại thời điểm đố còn chưa phức tạp như ngầy nay, nên việc sử dụng assembly để lập trình ứng dụng vẫn là một giải pháp phù hợp.

Nhưng hiện giờ đã là năm 2024, khi các MCU 32 bit đã rất rẻ và có nhiều bộ nhớ, các trình biên dịch ngôn ngữ C đã rất phổ biến và tối ưu, thư viện hỗ trợ cho các MCU cũng rất nhiều, điều này giúp cho lập trình nhúng trở nên dễ tiếp cận hơn. Tuy nhiên, với khoảng thời gian 10 năm làm nhúng tại nhiều công ty và được tiếp xúc với nhiều đồng nghiệp từ fresher cho tới các expert 20 năm kinh nghiệm, tôi quan sát thấy rằng những kỹ sư có kiến thức nền tảng và hiểu biết sâu sắc về hệ thống nhúng đều có khả năng đọc, hiểu và viết chương trình assembly.

Tôi sẽ nêu ra một vài trường hợp tại sao ngôn ngữ cấp cao (như C) không thể thay thế assembly.
* Khi dự án làm việc với một MCU mới, tài liệu và `C` compiler (bắt buộc phải có), và chỉ thế thôi, chúng ta không có IDE, không có project "Hello world", không gì cả.
* Khi bạn cần viết/sửa đổi core implementation của vài hệ điều hành (cụ thể hơn là RTOS).
* Khi compiler bị lỗi!!! Đúng vậy, việc compiler bị lỗi khá là phổ biến nếu bạn code cho embedded đủ nhiều, kể cả compiler có bản quyền nhé.
* Khi chúng ta muốn hiểu sâu về kiến trúc của một MCU, và đây là điều quan trọng nhất.

# Nhật ký của một kỹ sư phần mềm nhúng.
##  Đừng bị lầm tưởng, bởi sự dễ dàng của Arduirno!
Vâng, với arduino, bạn có thể mua các module cảm biến và một bộ khung robot về và ... Voila, chúng ta đã có một project khá là cool. Bạn học được cách các sensor hoạt động như thế nào, bạn biết cách điều khiển các cánh quạt quay như thế nào để máy bay có thể bay lên , bay tiến hay bay lùi,... Cho đến khi bạn đi làm và bạn nhận ra, những dự án thực thụ không dùng arduino. Bạn được giao cho một dự án với một con MCU bạn chưa gặp bao giờ, dự án không có IDE, mọi người viết code bằng bất cứ text editor nào và mọi người build code bằng commandline. Bạn nhập cuộc khá dễ dàng, bạn follow guideline, và bạn đẫ build được project như mọi người. Tiếp theo, bạn được giao cho nhiệm vụ viết một driver cho một khối ngoại vi của dự án, bạn háo hức với lần đầu tiên bạn được thể hiện khả năng coding của mình. Bạn bắt đầu viết những dòng code đầu tiên, bạn được mọi người trong team hướng dẫn cho cách sử dụng debugger, mọi thứ khá suôn sẻ. Và chuyện gì rồi cũng sẽ đến, bạn thấy chương trình của mình không chạy, bạn muốn debug, bạn không có hàm print của arduino, bạn phải làm quen với việc sử dụng debugger, cũng không phải việc gì quá khó khăn, ngoại trừ việc bạn  thấy chương trình của mình viết chạy không theo thứ tự từ trên xuống dưới như bạn từng biết. Với tâm thế cẩn thận, thử lại nhiều lần, bạn nghĩ rằng debugger bị hư, hoặc con MCU bị hư, vì nó chạy không đúng thứ tự, bạn báo cáo với mentor của bạn. Mentor của bạn  mỉm cười và nói "Chương trình của mình build với compiling option -O2, nó sẽ optimize code bằng cách re-order (sắp xếp lại) các instruction", "Vậy làm sao đểu build không optimize vậy anh?", "Ở đây chúng ta không làm thế, debug assembly đi em". WHATTT??? Assembly, ngôn ngữ tối cổ chỉ học 4 buổi trên trường đại học và bạn còn chẳng nhớ bạn đã từng học cái gì.

## Khi bạn là người đầu tiên trong công ty làm việc với một dòng MCU mới/lạ.
Bạn được công ty giao cho một dự án mới, với những công nghệ mới nhất, con MCU và kit phát triển bạn được giao là bản prototype chỉ giành cho những partner có ký NDA, bạn không thấy có một theard stackoverflow nào nói về dòng MCU này. Và công việc của bạn khá đơn giản: "Bring up board này cho chạy tới hàm main". Bạn không có gì nhiều ngoài tài liệu của dòng MCU này (bản preliminary strictly confidential), compiler lưu hành nội bộ. Không có "BSP - Board Support Package", vì bạn là người tạo ra nó. Và bạn phải bắt đầu tìm hiểu về cách con MCU này start up như nào và bạn biết rằng `C` compiler bạn đang có không có một cách chính thống nào để thiết lập được Stack Pointer trước khi project của bạn có thể sử dụng được chương trình viết bằng ngôn ngữ C.
Bạn tìm hiểu cách viết chương trình bằng assembly, và bạn ước bạn đã học assembly kỹ hơn, bạn thấy mọi thứ được dạy trên trường đại học thật là hợp lý. Bạn chợt ngờ ngợ hiểu ra Stack Pointer là gì, Vector table là gì, Program counter là gì,...

## Khi bạn có một project làm việc với co-processor và không có C compiler.
What da heck? Tại sao lại không có C compiler? Tại vì co-processor thuộc về những tập ứng dụng ngách dẫn đến việc những công ty làm compiler không muốn đầu tư để làm C compiler, chỉ đơn giản là lý do liên quan đến cost. Và bạn thấy lúc này, bạn thấy rằng chẳng có gì khó khăn cả, bạn thoải với việc viết cả một chương trình lớn bằng assembly.

## Bạn gặp exception.
Đôi khi, bạn thấy chương trình của bạn gây ra exception, và bạn không hiểu tại sao, chương trình của bạn nhìn khá logic. Nhưng khi debug assembly, bạn nhận ra rằng bằng cách nào đó, mỗi khi chương trình của bạn gọi đến một instrucion cụ thể, sau đó bạn thấy gây ra exception liên quan đến memory. Bạn nhìn cách CPU register và nhận ra rằng instruction của bạn truy cập vào "non-aligment" memory address. Voila, đây là lúc bạn đã hiểu việc học/đọc/hiểu ngôn ngữ ASSEMBLY có ích thế nào.

# Lời kết,
4 tình huống tôi kể trên là tình huống giả định về một kỹ sư (in general) gặp những hoàn cảnh cần kiến thức về assembly trong công việc, từ lúc là junior, qua mid level, cho đến lúc là "hardcore" engineer. Chắc chắn rằng việc hiểu toàn bộ các câu lệnh hợp nữ có thể không cần thiết ở thời điểm tôi viết bài viết này, nhưng việc nắm được cách đọc và hiể assembly sẽ giúp chúng ta rất nhiều trong việc debug và làm việc với low level software, và chắc chắn rằng không gì có thể ngăn cản bạn thiết kế ra chương trình cho bất kể MCU nào.

# Một số gợi ý cách tiếp cận ngôn ngữ assembly.
* Build-from-scratch một project với bất kỳ evaluation kit nào bạn đang có. Giả sử bạn chỉ có toolchain (compiler, assembler, linker), bạn thử build một project "hello word" như blink led. Bạn thử tập viết startup code, viết linker script,...
* Thử thiết kế và viết một RTOS đơn giản.
* Viết một chương trình xuất xung PWM bằng assembly (toàn bộ).

