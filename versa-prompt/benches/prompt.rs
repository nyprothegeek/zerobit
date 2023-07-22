use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::vec;
use versa_prompt::{map, FinalizablePrompt, Prompt, PromptList, Role, Tag};

fn prompt_create(c: &mut Criterion) {
    c.bench_function("prompt_create", |b| {
        b.iter(|| {
            PromptList::new(
                black_box("You are a robot assistant for making personalized greetings."),
                vec![],
            );

            Prompt::new(black_box(
                "You are a robot assistant for making personalized greetings.",
            ))
        })
    });
}

fn prompt_add_message(c: &mut Criterion) {
    let mut prompt_map = PromptList::new(
        "You are a robot assistant for making personalized greetings.",
        vec![],
    );

    c.bench_function("prompt_add_message", |b| {
        b.iter(|| {
            prompt_map.add_message(
                black_box("Can you help me make a personalized greeting?"),
                black_box(vec![Tag::Role(Role::User)]),
            )
        })
    });
}

fn prompt_format(c: &mut Criterion) {
    let mut prompt = Prompt::new("You are a robot assistant for making {{function}}.");
    let mut prompt_map =
        PromptList::new("You are a robot assistant for making {{function}}.", vec![]);

    c.bench_function("prompt_format", |b| {
        b.iter(|| {
            prompt
                .format(black_box(map!("function" => "personalized greetings")))
                .unwrap();
            prompt_map
                .format(black_box(map!("function" => "personalized greetings")))
                .unwrap();
        })
    });
}

criterion_group!(benches, prompt_create, prompt_add_message, prompt_format);
criterion_main!(benches);
