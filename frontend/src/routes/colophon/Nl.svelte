<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course. © Copyright Utrecht University (Department of
    Information and Computing Sciences)
-->

<script lang="ts">
    import { m } from '$lib/paraglide/messages.js';
    import uuLogo from '$lib/assets/uu-logo-nl.svg';
    import huLogo from '$lib/assets/hu-logo-nl.svg';
</script>

<div>
    <h1 class="page__title">{m.colophon()}</h1>

    <p class="pb-2">
        Deze webapplicatie is het resultaat van een onderzoeksproject van de volgende twee personen:
    </p>
    <div class="grid grid-cols-1 justify-items-center gap-8 text-center md:grid-cols-2">
        <div>
            <img src={uuLogo} alt="Logo Universiteit Utrecht" class="mx-auto mb-4" />
            <p>
                <b>Rogier Bos</b><br />Freudenthal Instituut<br /><a
                    href="https://www.uu.nl/medewerkers/RDBos"
                    class="text__link"
                    target="_blank"
                    rel="external">website</a
                ><br /><a
                    href="mailto:r.d.bos@uu.nl"
                    class="text__link"
                    target="_blank"
                    rel="external">r.d.bos@uu.nl</a
                >
            </p>
        </div>

        <div>
            <img src={huLogo} alt="Logo Hogeschool Utrecht" class="mx-auto mb-4" />
            <p>
                <b>Theo van den Bogaart</b><br />Archimedes Instituut, leerstoel Wiskundige en
                analytische vaardigheden van professionals<br /><a
                    href="mailto:theo.vandenbogaart@hu.nl"
                    class="text__link"
                    target="_blank"
                    rel="external">theo.vandenbogaart@hu.nl</a
                >
            </p>
        </div>
    </div>
    <p class="pt-2 pb-2">
        en ontwikkeld door de volgende studenten van de bachelor Informatica aan de Universiteit
        Utrecht binnen het Software Project vak:
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
        onder begeleiding van <a
            href="https://www.uu.nl/medewerkers/RTRadulescu"
            class="text__link"
            target="_blank"
            rel="external">Dr. Roxana Radulescu</a
        >.
    </p>
    <p>
        Meer informatie over het Software Project vak is te vinden op <a
            href="https://softwareprojecten.sites.uu.nl/"
            class="text__link"
            target="_blank"
            rel="external">https://softwareprojecten.sites.uu.nl</a
        >.
    </p>
    <p class="pt-5">© Copyright Universiteit Utrecht (Departement Informatica)</p>
</div>
