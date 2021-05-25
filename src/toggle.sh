POMOID=$(pidof pomodoro)
STATE=$(ps -q $POMOID -ao state)

if [ "${STATE:2:1}" = "S" ] ; then
    kill -STOP $POMOID
    echo "Kill"
else
    kill -CONT $POMOID
    echo $STATE
    echo "Cont"
fi
