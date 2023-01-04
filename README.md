 
<p align="center">
<img  src="https://github.com/berryes/TikTunnel/blob/main/logo.png?raw=true"  width="160">
</p>

<h1 align="center"> TikTunnel 

<br>


 ![GitHub issues](https://img.shields.io/github/issues/berryes/tiktunnel?color=%23BF1363&style=for-the-badge) ![GitHub last commit](https://img.shields.io/github/last-commit/berryes/tiktunnel?color=%23BF1363&logoColor=%23BF1363&style=for-the-badge) ![GitHub](https://img.shields.io/github/license/berryes/tiktunnel?color=%23BF1363&logoColor=%23BF1363&style=for-the-badge)

</h1>


An API for scraping tiktoks written in Rust!

This program scrapes data from Tiktok and serves an endpoint to safely view the content. 



## Todo / plans
Well first of all is actually writing the program. But neverless here are some features I want to implement in the near future. 

- [ ]  Finishing the roadmap 
- [ ] Ai for recommendation?
- [ ] Frontend (IOS/Android/Web/Windows/Linux)?


## Roadmap 

- [ ] Creating the scraper part.

- [ ] Removing the tiktok watermark
    - There is some way to do it. I'm not aware of such methods rigth now.

- [ ] Building the API via [Rocket](https://rocket.rs)
    - I have only built basic applications with rust so this is going to be a challange for me. I do have experience building fullstack apps with JS tho

- [ ] Overcoming being ip/request banned 
    <br>
    - With 
        - Randomizing the client infromation if possible (Browser type,Screen size, platform, buildnumber ect)

        - VPNs/Proxies
            via OpenVPN and reqwest proxy.


- Unsure / Migth abandon
    - [ ]   Creating workers
        - Workers are an essential part to a program when it comes to distrubuting the load.
        Many requests at a time can greatly slow the processing speed.



## Why Rust?
First, I was skeptycal about this language. But now after using it for a few projects I have realised its freaking great! Its blazing fast, the memory safety features make sense. So does the syntaxt! I could imagine myself being a Rust developer one day. +1 

# API

[![Run in Postman](https://run.pstmn.io/button.svg)](https://app.getpostman.com/run-collection/21747798-9ad12351-8864-489b-9bac-bbe175875adb?action=collection%2Ffork&collection-url=entityId%3D21747798-9ad12351-8864-489b-9bac-bbe175875adb%26entityType%3Dcollection%26workspaceId%3D456bdba7-0741-4ccb-a8ad-40dcec1eb317)
