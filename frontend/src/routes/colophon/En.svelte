<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course. © Copyright Utrecht University (Department of
    Information and Computing Sciences)
-->

<script lang="ts">
    import { m } from '$lib/paraglide/messages.js';
    import uuLogo from '$lib/assets/uu-logo-en.svg';
    import huLogo from '$lib/assets/hu-logo-en.png';
</script>

<div>
    <h1 class="page__title">{m.colophon()}</h1>
    <p class="pb-2">
        This web-application is the result of a research project of the following two persons:
    </p>
    <div class="grid grid-cols-1 justify-items-center gap-8 text-center md:grid-cols-2">
        <div>
            <img src={uuLogo} alt="Logo Utrecht University" class="mx-auto mb-4" />
            <p>
                <b>Rogier Bos</b><br />Freudenthal Institute<br /><a
                    href="https://www.uu.nl/staff/RDBos"
                    target="_blank">website</a
                ><br /><a href="mailto:r.d.bos@uu.nl">r.d.bos@uu.nl</a>
            </p>
        </div>

        <div>
            <img
                src={huLogo}
                alt="Logo Utrecht University of Applied Sciences"
                class="mx-auto mb-4"
            />
            <p>
                <b>Theo van den Boogaart</b><br />Archimedes Institute, lectorate Mathematical and
                analytical skills of professionals<br /><a href="mailto:theo.vandenbogaart@hu.nl"
                    >theo.vandenbogaart@hu.nl</a
                >
            </p>
        </div>
    </div>
    <p class="pt-2 pb-2">
        and has been developed by the following students from the Bachelor Computer Science at
        Utrecht University within the Software Project course:
    </p>
    <div class="pb-2 text-center">
        <p>Sander Bertram</p>
        <p>Ruben Driessen</p>
        <p>Danique van Eck</p>
        <p>Jedidja Kamerbeek</p>
        <p>Dide van Lubek</p>
        <p>Do Toan Nguyen</p>
        <p>Jonathan Quist</p>
        <p>William Rijksen</p>
        <p>Kevin Roos</p>
        <p>Milo van der Tier</p>
    </div>
    <p class="pb-2">
        with the guidance of <a href="https://www.uu.nl/staff/RTRadulescu" target="_blank"
            >Roxana Radulescu</a
        >.
    </p>
    <p>
        Further information about the Software Project course can be found on <a
            href="https://softwareprojecten.sites.uu.nl/"
            target="_blank">https://softwareprojecten.sites.uu.nl</a
        >.
    </p>
    <p class="pt-5">
        © Copyright Utrecht University (Department of Information and Computing Sciences)
    </p>
</div>
