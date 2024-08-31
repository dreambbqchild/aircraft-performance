$at_2300 = "495 510 530 550 570 590 615 640 665,1205 1235 1265 1300 1335 1370 1415 1455 1500,510 530 550 570 590 615 640 660 690,1235 1265 1300 1335 1370 1415 1455 1495 1540,530 550 570 590 615 635 660 685 710,1265 1300 1335 1370 1410 1450 1490 1535 1580,545 565 590 610 635 655 685 710 735,1295 1330 1370 1405 1445 1485 1535 1575 1620,565 585 610 630 655 680 705 730 760,1330 1365 1405 1440 1480 1525 1570 1615 1665";
@rows = split(/,/, $at_2300);

$is_ground_roll = 1;
@ground_rolls = ();
@temperatures = ("", "", "", "", "", "", "", "", "" );

foreach (@rows)
{
    while($_ =~ /(\d+)/g) {
        if($is_ground_roll) {
            push(@ground_rolls, "Distance($1");
        } else {
            my $index = 8 - $#ground_rolls;
            my $ground_roll = shift(@ground_rolls);
            $temperatures[$index] = "$temperatures[$index] $ground_roll, $1),";
        }
    }

    $is_ground_roll = 1 - $is_ground_roll;
}

foreach (@temperatures)
{
    my $line = substr $_, 0, -1;
    print("[$line],\n");
}