function onLoad() {
    installEventHandlers();
}

function installEventHandlers() {
    let basicFloors = document.querySelectorAll(".floor-basic");

    for (basicFloor of basicFloors) {
        console.log(basicFloor);
        basicFloor.addEventListener("click", onClickBasicFloor);
    }
}

function onClickBasicFloor(event) {
    let targetElt = event.currentTarget;
    let detailElt = targetElt.nextElementSibling;

    console.assert(detailElt);
    console.assert(detailElt.classList.contains("floor-detail"));

    detailElt.classList.toggle("visible");
}

if (document.readyState == "loading") {
    document.addEventListener("DOMContentLoaded", (event) => {
        onLoad();
    });
} else {
    onLoad();
}
