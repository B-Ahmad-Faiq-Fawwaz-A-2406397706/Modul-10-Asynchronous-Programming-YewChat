## Experiment 3.1 - Original Code

![](images/Screenshot%202026-05-15%20224338.png)

## Experiment 3.2 - Add Some Creativities to the Webclient

![](images/Screenshot%202026-05-15%20225610.png)

![](images/Screenshot%202026-05-15%20225555.png)

Pada experiment ini, saya menambahkan beberapa kreativitas pada webclient agar tampilan YewChat tidak hanya mengikuti bentuk dasar dari tutorial, tetapi juga terasa seperti aplikasi chat yang memiliki identitas visual sendiri. Halaman login diubah menjadi lebih ekspresif dengan judul yang lebih kuat, ikon chat berbasis SVG, kartu langkah singkat, dan kalimat pembuka yang menjelaskan bahwa ruang chat ini dibuat untuk bertukar ide dengan cepat. Bagian chat juga dibuat lebih kaya dengan sidebar user, statistik jumlah user dan pesan, status room, empty state ketika belum ada pesan, serta panel kecil bernama Spark ideas yang berisi inspirasi percakapan. Saya tetap mempertahankan konsep utama dari tutorial, yaitu komunikasi WebSocket dengan format JSON, sehingga perubahan ini lebih fokus pada pengalaman pengguna daripada mengubah alur data utama. Saya juga menambahkan handling yang lebih aman ketika menerima pesan dari WebSocket, sehingga client tidak langsung crash jika ada pesan yang tidak sesuai format JSON. Menurut saya, kreativitas di sini bukan hanya soal warna atau dekorasi, tetapi juga soal membuat pengguna lebih paham konteks aplikasi sejak pertama kali membuka halaman.

