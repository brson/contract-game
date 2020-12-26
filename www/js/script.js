"use strict";

function onLoad() {
    installEventHandlers();
}

function installEventHandlers() {
    let basicFloors = document.querySelectorAll(".floor-basic");

    for (let basicFloor of basicFloors) {
        basicFloor.addEventListener("click", onClickBasicFloor);
    }
}

function onClickBasicFloor(event) {
    let targetElt = event.currentTarget;
    let detailElt = targetElt.nextElementSibling;

    console.assert(detailElt);
    console.assert(detailElt.classList.contains("floor-detail"));

    detailElt.classList.toggle("visible");

    let floorElt = targetElt.parentNode;

    console.assert(floorElt);
    console.assert(floorElt.classList.contains("floor"));

    let towerElt = floorElt.parentNode;

    console.assert(towerElt);
    console.assert(towerElt.classList.contains("tower"));

    towerElt.classList.toggle("collapsed");

    console.log(towerElt.children);

    for (let floorElt of towerElt.children) {
        floorElt.classList.remove("top-stacked-floor");
    }

    floorElt.classList.add("top-stacked-floor");
}

if (document.readyState == "loading") {
    document.addEventListener("DOMContentLoaded", (event) => {
        onLoad();
    });
} else {
    onLoad();
}
