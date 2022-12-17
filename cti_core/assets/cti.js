"use strict";(()=>{const r=(n,e=0)=>{let a=3735928559^e,o=1103547991^e;for(let e=0,t;e<n.length;e++)t=n.charCodeAt(e),a=Math.imul(a^t,2654435761),o=Math.imul(o^t,1597334677);return a=Math.imul(a^a>>>16,2246822507)^Math.imul(o^o>>>13,3266489909),4294967296*(2097151&(o=Math.imul(o^o>>>16,2246822507)^Math.imul(a^a>>>13,3266489909)))+(a>>>0)},s=e=>{var t=document.createElement("a");return t.href="/users.html?name="+encodeURIComponent(e),t.innerText=e,t},n=e=>{if(200===e.status)return e.json();throw new Error("oops, couldn't load the data")},d=(e,t=0)=>{var n=new Intl.RelativeTimeFormat("en",{numeric:"always"}),e=(Date.now()-e)/1e3;return e<60?n.format((-e).toFixed(1),"seconds"):e<3600?n.format((-e/60).toFixed(0),"minutes"):e<86400?n.format((-e/3600).toFixed(0),"hours"):e<2678400?n.format((-e/86400).toFixed(0),"days"):e<31622400?n.format((-e/2592e3).toFixed(0),"months"):"over a year ago"},i=e=>{document.getElementById("last-updated").innerText=`(Last updated ${d(Date.parse(e+"Z"),1)})`};window.updateTops=e=>{var t=new URL(location.href);t.hash=e,location.replace(t),location.reload()};var e,t=()=>{fetch("/api/blocks").then(n).then(({blocks:e,last_updated_at:t})=>{var n=document.getElementById("segment"),a=document.getElementById("segments");a.innerHTML="",i(t);for(let t=0;t<256;t++){var o,l=n.content.cloneNode(!0).querySelector("*"),d=(a.appendChild(l),l.querySelector(".segment-cidr").innerText=t+".0.0.0/8",e.findIndex(e=>e.block===t));0<=d?(d=e[d],o=l.querySelector(".segment-nick"),l.querySelector(".segment-count").innerText=`(${d.claims} address${1==d.claims?"":"es"})`,d.is_tied?(o.innerText="(tied)",l.classList.add("segment-tied")):(o.appendChild(s(d.nick)),l.style.backgroundColor=(o=d.nick,`hsla(${(4095&r(o))/4095*180|0}, 90%, 90%, 1)`))):l.classList.add("segment-empty")}})},a=()=>{var e=new URLSearchParams(location.search).get("name"),t=document.getElementById("user-name");const r=document.getElementById("total-addresses"),s=document.querySelector("ul");t.innerText=e,fetch("/api/users?name="+e).then(n).then(({rankings:e,last_updated_at:t})=>{i(t),r.innerText=e.reduce((e,t)=>e+t.claims,0);for(const d of e){var n=document.createElement("li"),a=`${d.is_tied?"Tied ":""}${o=d.rank,a=void 0,l=void 0,a=o%10,l=o%100,1==a&&11!=l?o+"st":2==a&&12!=l?o+"nd":3==a&&13!=l?o+"rd":o+"th"} in block ${d.block}.0.0.0/8 with ${d.claims} address`+(1==d.claims?"":"es");n.innerHTML=a,s.appendChild(n)}var o,a,l})},o=location.pathname;"/"===o||"/index.html"===o?(fetch("/api/ip").then(n).then(e=>{e.v4&&(document.getElementById("user-ip").innerText=e.ip)}),fetch("/api/recent").then(n).then(({claims:e})=>{const l=document.getElementById("recent-claims"),t=document.getElementById("show-all-recent");l.innerHTML="",e.forEach(({nick:e,ip:t,claimed_at:n},a)=>{var o=document.createElement("div");o.classList.add("recent-claim"),o.append(`${t} got captured ${d(1e3*n)} by `),o.appendChild(s(e)),10<=a&&(o.style.display="none"),l.appendChild(o)}),t.style.display="",t.onclick=function(){t.style.display="none",l.querySelectorAll(".recent-claim").forEach(e=>e.style.display="")}}),e=document.getElementById("rank-period"),location.hash&&(e.value=location.hash.slice(1)),e={hour:"/hour",day:"/day",week:"/week",month:"/month",year:"/year",all:"/all"}[e.value],fetch("/api/ranks"+e).then(n).then(({rankings:e})=>{const l=document.querySelector("ol"),t=(e.forEach(({nick:e,blocks:t,total_claims:n},a)=>{var o=document.createElement("li");o.appendChild(s(e)),o.append(` with ${n} address${1==n?"":"es"}, and ${t.length} block${1<t.length?"s":""} controlled: `),o.append(t.join(", ")),l.appendChild(o)}),document.getElementById("show-all-tops"));0<e.length?(t.style.display="",t.onclick=function(){t.style.display="none",l.classList.remove("topN")}):(document.getElementById("no-claimers").innerText="No claimers for this period yet. Why not go ahead and capture some IPs?",t.style.display="none")}),t()):"/users.html"===o&&a()})();