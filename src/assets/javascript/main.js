$(document).ready(function() {
    /* $("button").click(function(){
     $("#test").hide();
     });*/

    $("#submitBtn").click(function () {

        var name = $("#name").val();
        var className = $("#classSelect").val();
        var raceName = $("#raceSelect").val();
        var strengthStat = $("#strength_stat").val() != "" ? $("#strength_stat").val() : 0;
        var dextirityStat = $("#dex_stat").val() != "" ? $("#dex_stat").val() : 0;
        var constitutionStat = $("#con_stat").val() != "" ? $("#con_stat").val() : 0;
        var intlStat = $("#intl_stat").val() != "" ? $("#intl_stat").val() : 0;
        var wsdmStat = $("#wsdm_stat").val() != "" ? $("#wsdm_stat").val() : 0;
        var charismaStat = $("#charisma_stat").val() != "" ? $("#charisma_stat").val() : 0;
        var strengthMod = $("#strength_mod").val() != "" ? $("#strength_mod").val() : 0;
        var dextirityMod = $("#dex_mod").val() != "" ? $("#dex_mod").val() : 0;
        var constitutionMod = $("#con_mod").val() != "" ? $("#con_mod").val() : 0;
        var intlMod = $("#intl_mod").val() != "" ? $("#intl_mod").val() : 0;
        var wsdmMod = $("#wsdm_mod").val() != "" ? $("#wsdm_mod").val() : 0;
        var charismaMod = $("#charisma_mod").val() != "" ? $("#charisma_mod").val() : 0;
        var ac = $("#ac").val() != "" ? $("#ac").val() : 0;

        var charInst = new Character(name, className,raceName,
                            strengthStat, dextirityStat, constitutionStat,
                            intlStat, wsdmStat, charismaStat,
                            strengthMod, dextirityMod, constitutionMod,
                            intlMod, wsdmMod, charismaMod, ac);
        var params = JSON.stringify(charInst);
        alert(JSON.stringify(charInst));
        create_character("http://localhost:9000/character", params);
    });
    
    var charc = [{"name":"dina"}];
    var Race = {
        Human: "Human",
        Dwarf: "Dwarf",
        Elf: "Elf",
        HalfElf: "Half-Elf"
    };

    var stats = {
        stat: undefined,
        modifier: undefined
    };

    $(".view-character").click(function() {
        alert($(this).data("id"));
        var id = $(this).data("id");
        get_character("http://localhost:9000/character/id", id);
        // var element = $(".view-character").data("id");

    });

    function Character(name, className,raceName,
                       strengthStat, dextirityStat, constitutionStat,
                       intlStat, wsdmStat, charismaStat,
                       strengthMod, dextirityMod, constitutionMod,
                       intlMod, wsdmMod, charismaMod, ac) {

        var character = {
            id:0,
            name: name,
            class: className,
            race: raceName,
            strength_stat: strengthStat,
            dextirity_stat: dextirityStat,
            constitution_stat: constitutionStat,
            intelligence_stat: intlStat,
            wisdom_stat: wsdmStat,
            charisma_stat: charismaStat,
            strength_mod: strengthMod,
            dex_mod: dextirityMod,
            con_mod:constitutionMod,
            intl_mod:intlMod,
            wsdm_mod: wsdmMod,
            charisma_mod: charismaMod,
            ac: ac
        };

        return character;
    }

    function create_character(url,params){
        var http = new XMLHttpRequest();
        var url = url;
        var params = params;
        http.open("POST", url, true);
        http.onreadystatechange = function() {
            //Call a function when the state changes.
            if(http.readyState == 4 && http.status == 200) {
                //alert(http.responseText);

                console.log(http.responseText);
            }
        }
        http.send(params);
    };

    // function get_character(url, id) {
    //
    //     var http = new XMLHttpRequest();
    //     var url = url;
    //     var params = {
    //         id: id
    //     };
    //     http.open("GET", url + "?id="+id, true);
    //     http.onreadystatechange = function() {
    //         //Call a function when the state changes.
    //         if(http.readyState == 4 && http.status == 200) {
    //             console.log(http.responseText);
    //         }
    //     };
    //     http.send();
    // }
});