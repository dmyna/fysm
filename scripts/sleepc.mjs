#!/usr/bin/env -S node

import { createInterface } from "readline";
(async () => {
    var sleepPoints = 100;

    // SleepTimes
    const maxAcceptableSleepTime = 720;
    const idealSleepTime = 600;
    const reasonableSleepTime = 480;
    const minSleepTime = 360;

    const wrongValueWarn = "Você inseriu um valor errado! Refaça a questão.";
    const negativeRealTimeWarn =
        "Parece que você ficou acordado mais tempo do que você dormiu..." +
        "Mas pera... Isso é possível? Reinicie o script e tente de novo.";
    const beforeAsleepQuestion =
        "Quantos minutos demoraram até que você adormecesse? ";
    const timeInBedQuestion =
        "Por quantos minutos você esteve na cama até de fato acordar? ";

    const rl = createInterface({
        input: process.stdin,
        output: process.stdout,
    });

    const getTimeInBed = async () => {
        const answer = parseInt(
            await new Promise((resolve) =>
                rl.question(timeInBedQuestion, resolve)
            )
        );

        if (answer < 0) {
            console.log(wrongValueWarn);
            return getTimeInBed();
        }

        return answer;
    };
    const getTimeBeforeAsleep = async () => {
        const answer = parseInt(
            await new Promise((resolve) =>
                rl.question(beforeAsleepQuestion, resolve)
            )
        );

        if (answer < 0) {
            console.log(wrongValueWarn);
            return getTimeBeforeAsleep();
        }

        return answer;
    };

    const calculateTimeInBedPoints = (points, time) => {
        const iAndARatio = time - idealSleepTime;
        const cycleRange = 10;

        const calculateCycles = (range) => {
            const cicleMinutes = 90;
            const punishmentPoints = 20;
            const minimalRange = cicleMinutes - range / 2;
            const maximalRange = cicleMinutes + range / 2;

            if (
                time % cicleMinutes >= minimalRange &&
                time % cicleMinutes <= maximalRange
            ) {
                return points;
            } else return points - punishmentPoints;
        };

        if (iAndARatio >= -15 && iAndARatio <= 15) {
            return points;
        } else if (
            (iAndARatio > 15 && time <= maxAcceptableSleepTime) ||
            (iAndARatio < -15 && time >= reasonableSleepTime)
        ) {
            points -= 15;
        } else if (time >= minSleepTime && time < reasonableSleepTime) {
            points -= 25;
        } else if (time > maxAcceptableSleepTime) {
            points -= 30;
        } else if (time < minSleepTime) {
            points -= 50;
        }

        points = calculateCycles(cycleRange);

        return points;
    };

    const calculateAcceptableTimeBeforeAsleepPoints = (sleepPoints, time) => {
        const acceptableTimeBeforeAsleep = 30;
        const idealTimeBeforeAsleep = 15;
        const punishmentPoints = 10;

        if (time <= idealTimeBeforeAsleep) {
            // Ignore :)
        } else if (time <= acceptableTimeBeforeAsleep) {
            sleepPoints -= punishmentPoints * 0.5;
        } else if (time <= acceptableTimeBeforeAsleep * 1.5) {
            sleepPoints -= punishmentPoints * 1.5;
        } else if (time <= acceptableTimeBeforeAsleep * 2) {
            sleepPoints -= punishmentPoints * 2;
        } else {
            sleepPoints -= punishmentPoints * 3;
        }

        return sleepPoints;
    };

    var timeInBed;
    var timeBeforeAsleep;

    if (process.argv.length === 4) {
        timeInBed = parseInt(process.argv[2]);
        timeBeforeAsleep = parseInt(process.argv[3]);
    } else {
        timeInBed = await getTimeInBed();
        timeBeforeAsleep = await getTimeBeforeAsleep();
    }

    const realTime = timeInBed - timeBeforeAsleep;

    if (realTime < 0) {
        console.log(negativeRealTimeWarn);
        process.exit(1);
    }

    sleepPoints = calculateAcceptableTimeBeforeAsleepPoints(
        sleepPoints,
        timeBeforeAsleep
    );

    sleepPoints = calculateTimeInBedPoints(sleepPoints, realTime);

    console.log(sleepPoints);
    process.exit(0);
})();
