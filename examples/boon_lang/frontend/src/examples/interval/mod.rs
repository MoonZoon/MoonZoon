use crate::boon::platform::browser::preludes::for_generated_code::{println, *};

#[allow(dead_code)]
pub async fn run() -> Arc<Object> {
    let program = include_str!("interval.bn");
    println!("{program}");

    Object::new_arc(
        ConstructInfo::new(0, "root"),
        [Variable::new_arc(
            ConstructInfo::new(1, "document"),
            "document",
            TaggedObject::new_arc_value_actor(
                ConstructInfo::new(6, "Duration[..]"),
                RunDuration::Nonstop,
                "Duration",
                [Variable::new_arc(
                    ConstructInfo::new(7, "Duration.seconds"),
                    "seconds",
                    Number::new_arc_value_actor(
                        ConstructInfo::new(8, "Duration.seconds number"),
                        RunDuration::Nonstop,
                        1,
                    ),
                )],
            )
            .pipe_to(|piped| {
                FunctionCall::new_arc_value_actor(
                    ConstructInfo::new(5, "Timer/interval(..)"),
                    RunDuration::Nonstop,
                    function_timer_interval,
                    [piped],
                )
                .pipe_to(|piped| {
                    ThenCombinator::new_arc_value_actor(
                        ConstructInfo::new(4, "THEN"),
                        RunDuration::Nonstop,
                        piped,
                        || Number::new_constant(ConstructInfo::new(9, "Number 1"), 1),
                    )
                    .pipe_to(|piped| {
                        FunctionCall::new_arc_value_actor(
                            ConstructInfo::new(3, "Math/sum(..)"),
                            RunDuration::Nonstop,
                            function_math_sum,
                            [piped],
                        )
                        .pipe_to(|piped| {
                            FunctionCall::new_arc_value_actor(
                                ConstructInfo::new(2, "Document/new(..)"),
                                RunDuration::Nonstop,
                                function_document_new,
                                [piped],
                            )
                        })
                    })
                })
            }),
        )],
    )
}
