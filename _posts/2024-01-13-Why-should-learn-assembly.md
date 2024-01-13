---
title:  "Tại sao kỹ sư hệ thống nhúng nên học ngôn ngữ assembly?"
categories:
   - Engineering
   - Vietnamese
tags:
---
**Info Notice:** Đây là bài viết đầu tiên tôi viết bằng tiếng Việt với mong muốn bài viết này đến được với nhiều đọc giả còn đang là sinh viên hoặc những kỹ sư trẻ đang làm ngành phần mềm nhúng. Bài viết này dựa trên bài viết [EABI - Why any embedded software engineer should know about it.](https://rtos.dev/engineering/EABI/). Mặc dù tôi đã cố gắng sử dụng nhiều từ vựng tiếng Việt nhất có thể, nhưng vì giới hạn của từ tiếng Việt đối với từ ngữ chuyên ngành nên tôi sẽ dùng từ ngữ tiếng Anh thay thế để cho giọng văn nghe sẽ tự nhiên với người đọc là những người làm kỹ sư hơn. .
{: .notice--info}


# Tại sao kỹ sư hệ thống nhúng nên học assembly?
Tôi vốn không học ngành khoa học máy tính hay những ngành liên quan đến lập trình, nhưng may mắn tôi được học khóa kiến trúc máy tính và hợp ngữ (`Assembly`) khi còn học đại học, và may mắn hơn nữa khi tôi được học với một người thầy lớn tuổi và có nhiều kinh nghiệm với những CPU "cổ" (ví dụ như 8086). Có lẽ vì thầy lớn tuổi nên thầy đã trả qua một kỳ khi mà các ngôn ngữ cấp cao chưa mạnh, hay các MCU tại thời điểm thầy làm việc có bộ nhớ khá khiêm tốn, và thực sự thì các ứng dụng sử dụng MCU tại thời điểm đó còn chưa phức tạp như thời điểm viết bài viết này nên việc sử dụng `assembly` để lập trình ứng dụng vẫn là một công việc phổ biến và phù hợp thời ấy.

Nhưng hiện giờ đã là năm 2024, khi các MCU 32-bit đã rất rẻ và có nhiều bộ nhớ, các trình biên dịch ngôn ngữ `C` (`C` compiler) đã rất phổ biến và tối ưu, thư viện hỗ trợ cho các MCU cũng đã được phát triển rất nhiều, điều này giúp cho việc lập trình nhúng trở nên dễ tiếp cận hơn. Tuy nhiên, với khoảng thời gian 10 năm làm nhúng tại nhiều công ty và được tiếp xúc với nhiều đồng nghiệp từ các bạn mới tốt ngiệp (fresher) cho tới các expert có hơn 10 năm kinh nghiệm, tôi quan sát thấy rằng những kỹ sư có kiến thức nền tảng vững vàng và hiểu biết sâu sắc về hệ thống nhúng đều có khả năng đọc, hiểu và viết chương trình `assembly`.

Tôi sẽ liệt kê ra một vài trường hợp tại sao ngôn ngữ cấp cao (như `C`) không thể thay thế `assembly`.
* Khi dự án làm việc với một MCU mới, chúng ta chỉ có tài liệu (Datasheet, user manual) và `C` compiler (bắt buộc phải có), và chỉ thế thôi, chúng ta không có IDE, không có BSP (Board Support Package), mọi thứ phải bắt đầu từ con số 0 (build from scratch).
* Khi bạn cần viết/sửa đổi core implementation của hệ điều hành (Việc này cũng khá phổ biến nếu dự án làm việc với in-house RTOS).
* Khi compiler bị lỗi!!! Đúng vậy, việc compiler bị lỗi khá là phổ biến nếu bạn code cho embedded đủ nhiều, kể cả compiler có bản quyền đắt tiền (Không một phần mềm nào là không có lỗi).
* Khi chúng ta muốn hiểu sâu về kiến trúc của một MCU/CPU, và **đây là điều quan trọng nhất**.

Và ở phần tiếp theo, tôi sẽ trích lại một hồi ký của một kỹ sư lập trình nhúng giả tưởng, qua đó phản ánh cách mà assembly xuất hiện trong công việc của một kỹ sư lập trình nhúng. Đó có thể là tôi hoặc bạn trong những giai đoạn khác nhau của công việc, và mục đích của tôi là để nhấn mạnh việc cần thiết phải biết về ngôn ngữ assembly kể cả khi chúng ta hiếm khi sử dụng nó, nhưng khi chúng ta cần tới nó, chúng ta biết sẽ phải làm gì.

# Nhật ký của một kỹ sư phần mềm nhúng.
##  Sức hấp dẫn bởi sự dễ dàng của Arduino!
*- Ngày XX tháng YY năm ZZZZ*

Vâng, với Arduino, bạn có thể mua các module cảm biến và một bộ khung robot về và ... Voila, chúng ta đã có một project khá là cool. Bạn học được cách các sensor hoạt động như thế nào, bạn biết cách điều khiển các cánh quạt quay như thế nào để máy bay có thể bay lên , bay tiến hay bay lùi, bạn có một đồ án tốt nghiệp đồ sộ đáng tự hào khi bạn đã lập trình những tính năng phức tạp nhất, tất nhiên là trên Arduino ... Cho đến khi bạn đi làm và bạn nhận ra, những dự án thực thụ không dùng Arduino. Bạn được giao cho một dự án với một con MCU bạn chưa gặp bao giờ, dự án không có IDE, mọi người viết code bằng bất cứ text editor nào (tất nhiên **Notepad++** luôn là sự lựa chọn số 1) và mọi người build code bằng command line (What the heck!!!). Bạn nhập cuộc khá dễ dàng, bạn follow coding guideline, và bạn đã build được project như mọi người. Tiếp theo, bạn được giao cho nhiệm vụ viết một driver cho một khối ngoại vi của dự án, bạn háo hức với lần đầu tiên bạn được thể hiện khả năng coding của mình. Bạn bắt đầu viết những dòng code đầu tiên, bạn được mọi người trong team hướng dẫn cho cách sử dụng debugger, mọi thứ khá suôn sẻ. Và chuyện gì rồi cũng sẽ đến, bạn thấy chương trình của mình không chạy, bạn muốn debug, bạn không có hàm print của arduino, bạn phải làm quen với việc sử dụng debugger, cũng không phải việc gì quá khó khăn, ngoại trừ việc bạn  thấy chương trình của mình viết chạy không theo thứ tự từ trên xuống dưới như bạn từng biết. Với tâm thế cẩn thận, thử lại nhiều lần, bạn nghĩ rằng debugger bị hư, hoặc con MCU bị hư, vì nó chạy không đúng thứ tự, bạn báo cáo với mentor của bạn. Mentor của bạn  mỉm cười và nói "Chương trình của mình build với compiling option `-O2`, nó sẽ optimize code bằng cách re-order (sắp xếp lại) các instruction", "Vậy làm sao đểu build không optimize vậy anh?", "Ở đây chúng ta không làm thế, debug bằng assembly đi em". WHATTT??? Assembly, ngôn ngữ tối cổ chỉ học 4 buổi trên trường đại học và bạn còn chẳng nhớ bạn đã từng học cái gì (ah, thực ra là tôi vẫn nhớ hàm `mov` để chuyển data giữa các register ?!?!!.

## Khi bạn là người đầu tiên trong công ty làm việc với một dòng MCU mới/lạ.
*- Ngày XX tháng YY năm ZZZZ*

Bạn được công ty giao cho một dự án mới, với những công nghệ mới nhất, con MCU và kit phát triển bạn được giao là bản prototype chỉ giành cho những partner có ký NDA (Non-Disclosure Agreement - thỏa thuận bảo mật thông tin), bạn không thấy có một theard stackoverflow hay post reddit nào nói về dòng MCU này. Và công việc của bạn khá đơn giản: "Bring-up (làm cho chạy) board này cho chạy tới hàm main". Bạn không có gì nhiều ngoài tài liệu của dòng MCU này (bản preliminary và strictly confidential), compiler lưu hành nội bộ (và còn có nhiều limitation haha). Không có "BSP - Board Support Package", vì bạn là người tạo ra nó. Và bạn phải bắt đầu tìm hiểu về cách con MCU này start up như nào và bạn biết rằng `C` compiler bạn đang dùng không có một cách chính thống nào để thiết lập được Stack Pointer trước khi project của bạn có thể sử dụng được chương trình viết bằng ngôn ngữ C.
Bạn tìm hiểu cách viết chương trình bằng assembly, và bạn ước bạn đã học assembly kỹ hơn, bạn thấy mọi thứ được dạy trên trường đại học thật là hợp lý. Bạn chợt ngờ ngợ hiểu ra Stack Pointer là gì, Vector Table là gì, Program counter là gì,...

## Khi bạn có một project làm việc với co-processor và không có C compiler.
*- Ngày XX tháng YY năm ZZZZ*

What da heck? Tại sao lại không có C compiler? Tại vì co-processor này dành ứng dụng ngách (niche), ai lại bỏ công sức tạo ra một C compiler mà dùng cho tập khách hàng nhỏ nhỉ? Đó chỉ chỉ đơn giản là lý do liên quan đến cost. Lúc này, bạn thấy rằng chẳng có gì khó khăn cả, bạn thoải với việc viết cả một chương trình lớn bằng assembly. Bạn bẻ tay, đứng dậy vặn người và ngồi xuống đọc tài liệu ISA của co-processor (ISA - Instruction Set Architecture - Kiến trúc tập lệnh), YOLO.

## Bạn gặp exception.
*- Ngày XX tháng YY năm ZZZZ*

Đôi khi, bạn thấy chương trình của bạn gây ra exception (exception là một dạng interrupt - ngắn- nhưng thường gây ra bởi lỗi của CPU, ví dụ như access (truy cập) địa chỉ không tồn tại), và bạn không hiểu tại sao, chương trình của bạn nhìn khá logic. Nhưng khi debug assembly, bạn nhận ra rằng bằng cách nào đó, mỗi khi chương trình của bạn gọi đến một instrucion cụ thể, sau đó bạn dùng debugger để stepping thì bạn thấy CPU gây ra exception liên quan đến memory. Bạn nhìn các CPU register và các thông tin về mã lỗi và nhận ra rằng instruction của bạn truy cập vào "non-aligment" memory address. Voila, đây là lúc bạn đã hiểu việc học/đọc/hiểu ngôn ngữ ASSEMBLY có ích thế nào. Một khi bạn hiểu được nguyên nhân gây ra vấn đề (root cause), việc sửa lỗi trở nên dễ như ăn bánh.

# Lời kết,
4 tình huống tôi kể trên là tình huống giả định về một kỹ sư (in general) gặp những hoàn cảnh cần kiến thức về assembly trong công việc, từ lúc là junior, qua mid level, cho đến lúc là "hardcore" engineer. Chắc chắn rằng việc nhớ toàn bộ các câu lệnh hợp ngữ có thể không cần thiết ở thời điểm tôi viết bài viết này, nhưng việc nắm được cách đọc và hiểu assembly sẽ giúp chúng ta rất nhiều trong việc debug và làm việc với low level software, và chắc chắn rằng không gì có thể ngăn cản bạn thiết kế ra chương trình cho bất kể MCU nào.

## Một số gợi ý cách tiếp cận ngôn ngữ assembly
* Build-from-scratch một project với bất kỳ evaluation kit nào bạn đang có. Giả sử bạn chỉ có toolchain (compiler, assembler, linker), bạn thử build một project "hello word" như blink led. Bạn thử tập viết startup code, viết linker script,...
* Thử thiết kế và viết một RTOS đơn giản.
* Viết một chương trình xuất xung PWM bằng assembly.

## Một vài link tham khảo các khái niệm trong bài viết
* [Memory reordering - Wikipedia](https://en.wikipedia.org/wiki/Memory_ordering)
* [GCC assembly syntax](https://www.felixcloutier.com/documents/gcc-asm.html)
* [ARM Cortex M exception Handling - Memfault](https://interrupt.memfault.com/blog/arm-cortex-m-exceptions-and-nvic)

--
