import React from 'react';
import ImageGallery from 'react-image-gallery';
import styles from './styles.module.css';
import "react-image-gallery/styles/css/image-gallery.css";

export function VideoGallery({ }) {
    return (
        <iframe src="//www.youtube.com/embed/x8LwVi2LrO4" frameborder="0" allowfullscreen></iframe>
    );
}

const images = [
    {
        filename: "wipeoutpure.jpg",
        description: "Wipeout Pulse",
    },
    {
        filename: "burnoutdominator.jpg",
        description: "Burnout Dominator",
    },
    {
        filename: "persona3portable.jpg",
        description: "Persona 3 Portable",
    },
    {
        filename: "ridgeracer2.jpg",
        description: "Ridge Racer II",
    },
    {
        filename: "finalfantasycrisiscore.jpg",
        description: "Crisis Core: Final Fantasy",
    },
    {
        filename: "gtavcs.jpg",
        description: "Grand Theft Auto - Vice City Stories",
    },
    {
        filename: "monsterhunterfreedomunite.jpg",
        description: "Monster Hunter Freedom",
    },
    {
        filename: "granturismo.jpg",
        description: "Gran Turismo",
    },
    {
        filename: "littlebigplanet.jpg",
        description: "Little Big Planet",
    },
    {
        filename: "gitarooman.jpg",
        description: "Gitaroo Man",
    },
    {
        filename: "acecombatx.jpg",
        description: "Ace Combat X",
    },
    {
        filename: "soulcalibur.jpg",
        description: "Soul Calibur",
    },
    {
        filename: "bustamovedeluxe.jpg",
        description: "Bust-a-move Deluxe",
    },
    {
        filename: "daxter.jpg",
        description: "Daxter",
    },
    {
        filename: "disgaea2.jpg",
        description: "Disgaea 2",
    },
    {
        filename: "exit.jpg",
        description: "Exit",
    },
    {
        filename: "ghostsandgoblins.jpg",
        description: "Ghosts & Goblins",
    },
    {
        filename: "gtalcs.jpg",
        description: "Grand Theft Auto - Liberty City Stories",
    },
    {
        filename: "gtalcs2.jpg",
        description: "Grand Theft Auto - Liberty City Stories",
    },
    {
        filename: "lemmings.jpg",
        description: "Lemmings",
    },
    {
        filename: "lumines.jpg",
        description: "Lumines",
    },
    {
        filename: "mimana.jpg",
        description: "Mimana",
    },
    {
        filename: "mortalkombat.jpg",
        description: "Mortal Kombat",
    },
    {
        filename: "patapon3.jpg",
        description: "Patapon 3",
    },
    {
        filename: "swordartonline.jpg",
        description: "Sword Art Online",
    },
    {
        filename: "worms.jpg",
        description: "Worms",
    },
];

function genUrls(images) {
    return images.map((image) => {
        return {
            original: "/static/img/screenshots/" + image.filename,
            thumbnail: "/static/img/screenshots/thumbs/" + image.filename,
            description: image.description,
        };
    });
}

export default function ScreenshotGallery({ }) {
    return (
        <>
            <section>
                <row>
                    <ImageGallery
                        lazyLoad="true"
                        items={genUrls(images)}
                    />
                </row>
            </section>
        </>
    );
}
