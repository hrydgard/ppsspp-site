function onFSPopupClosed(orderReference) {
    if (orderReference) {
        console.log("Order completed - redirecting!");
        console.log(orderReference.reference);
        fastspring.builder.reset();
        var url = "/thankyou?orderId=" + orderReference.reference;
        try { window.location.replace(url); }
        catch (e) { window.location = url; }
    } else {
        console.log("no order ID - won't redirect to thankyou page");
    }
}
