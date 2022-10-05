import React, { version } from 'react';

// This omits
//    <script async src="https://pagead2.googlesyndication.com/pagead/js/adsbygoogle.js?client=ca-pub-3281131109267988"
//    crossorigin="anonymous"></script>
// since we already put it in the header.
// Should probably not use these inbetween ads, they're kinda bad.
export function AdInBetween() {
  return (
    <>
<ins className="adsbygoogle"
     style={{display:"block"}}
     data-ad-format="fluid"
     data-ad-layout-key="-h4+a+f-6f+9l"
     data-ad-client="ca-pub-3281131109267988"
     data-ad-slot="9626096124"></ins>
<script>
     (adsbygoogle = window.adsbygoogle || []).push({});
</script>
</>
  )
}

export default function AdResponsive() {
  return (
    <>
    {/* Responsive Black -->*/}
    <ins className="adsbygoogle"
        style={{display:"block"}}
        data-ad-client="ca-pub-3281131109267988"
        data-ad-slot="6642932432"
        data-ad-format="auto"
        data-full-width-responsive="true"></ins>
    <script>
        (adsbygoogle = window.adsbygoogle || []).push({});
    </script>
    </>
  );
}
